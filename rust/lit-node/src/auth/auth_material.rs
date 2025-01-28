use crate::{
    config::LitNodeConfig as _,
    constants::{
        Chain, CHAIN_CHEQD, CHAIN_COSMOS, CHAIN_ETHEREUM, CHAIN_JUNO, CHAIN_KYVE, CHAIN_SOLANA,
    },
    error::{conversion_err, unexpected_err, validation_err, validation_err_code, Result, EC},
    utils::{chain, encoding, web::EndpointVersion},
};
use core::result::Result as CoreResult;
use ethers::types::Address;
use lit_core::{config::LitConfig, error::Unexpected};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::{fmt, sync::Arc};

#[allow(deprecated)]
use super::{
    contract::validate_eip1271_signature,
    lit_resource::LitResource,
    resources::LitResourceAbility,
    session_sigs::{
        extract_requested_resources_from_session_sig, extract_wallet_sig,
        validate_and_extract_wallet_sig, validate_session_sig,
    },
    validators::wallet_sig::{validate_siwe_message, validate_wallet_sig_by_chain},
};

pub const AUTH_SIG_DERIVED_VIA_SESSION_SIG: &str = "litSessionSignViaNacl";
pub const AUTH_SIG_DERIVED_VIA_BLS_NETWORK_SIG: &str = "lit.bls";
pub const AUTH_SIG_DERIVED_VIA_CONTRACT_SIG: &str = "EIP1271";
pub const AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256: &str = "EIP1271_SHA256";
pub const AUTH_SIG_SESSION_SIG_ALGO: &str = "ed25519";
pub const AUTH_SIG_BLS_NETWORK_SIG_ALGO: &str = "LIT_BLS";

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AuthMaterialType {
    #[default]
    /// This is an auth sig that was derived via a wallet.
    WalletSig,

    /// This is an auth sig that was derived via EIP 1271.
    ContractSig,

    /// This is an auth sig that was derived via session keys.
    SessionSig,

    /// This is an auth sig that was signed by the BLS network key
    BLSNetworkSig,
}

/// ValidatedAddress is an address that has been validated against the corresponding chain.
///
/// Note: It can only be obtained by calling `validate_and_get_user_address`.
#[derive(Debug)]
pub struct ValidatedAddress {
    address_str: String,
    chain: Chain,
}

impl ValidatedAddress {
    pub(self) fn new(address_str: String, chain: Chain) -> Self {
        Self { address_str, chain }
    }

    pub fn address_str(&self) -> &String {
        &self.address_str
    }

    /// Check whether the address is an EVM-compatible address.
    pub fn is_evm_user_address(&self) -> bool {
        chain::is_evm_compatible_chain(&self.chain)
    }

    /// Get the EVM-compatible address.
    ///
    /// Note: This function will return `Err` if the address is not an EVM-compatible address.
    pub fn evm_address(&self) -> Result<Address> {
        if !self.is_evm_user_address() {
            return Err(unexpected_err(
                "Address is not EVM-compatible",
                Some("Unable to get EVM address".into()),
            ));
        }
        Ok(Address::from_slice(
            &encoding::hex_to_bytes(self.address_str()).map_err(|e| {
                conversion_err(e, Some("Error converting hex string to bytes".into()))
            })?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AuthSigItem {
    Single(JsonAuthSig),
    Multiple(MultipleAuthSigs),
}

impl AuthSigItem {
    pub(crate) async fn validate_and_get_user_address(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        chain: &Option<String>,
        cfg: &LitConfig,
        bls_root_pubkey: &String,
        endpoint_version: &EndpointVersion,
    ) -> Result<ValidatedAddress> {
        let valid_wallet_sig = {
            match self {
                AuthSigItem::Multiple(multiple_auth_sigs) => {
                    debug!("multiple sigs in auth_sig_item");
                    multiple_auth_sigs
                        .validate_all_and_extract_single_wallet_sig(
                            requested_lit_resource_ability,
                            cfg,
                            bls_root_pubkey,
                            endpoint_version,
                        )
                        .await
                        .map_err(|e| {
                            validation_err(
                                e,
                                Some("Unable to extract wallet sig from multiple auth sigs".into()),
                            )
                        })?
                }
                AuthSigItem::Single(single_auth_sig) => {
                    debug!("single sig in auth_sig_item");
                    single_auth_sig
                        .validate_and_get_wallet_sig(
                            requested_lit_resource_ability,
                            chain,
                            cfg,
                            bls_root_pubkey,
                            endpoint_version,
                        )
                        .await
                        .map_err(|e| {
                            validation_err_code(
                                e,
                                EC::NodeInvalidAuthSig,
                                Some("Invalid AuthSig".into()),
                            )
                        })?
                }
            }
        };

        let user_address = valid_wallet_sig
            .user_address(bls_root_pubkey)
            .await
            .map_err(|e| {
                unexpected_err(e, Some("Unable to get user address from wallet sig".into()))
            })?;
        let chain = valid_wallet_sig
            .chain()
            .ok_or(unexpected_err("Unable to get chain from wallet sig", None))?;
        Ok(ValidatedAddress::new(user_address, chain.to_owned()))
    }

    pub(crate) fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        match self {
            AuthSigItem::Multiple(multiple_auth_sigs) => multiple_auth_sigs.resources(),
            AuthSigItem::Single(single_auth_sig) => single_auth_sig.resources(),
        }
    }

    pub fn get_auth_type(&self) -> Result<&AuthMaterialType> {
        match self {
            AuthSigItem::Single(json_auth_sig) => Ok(&json_auth_sig.auth_material_type),
            AuthSigItem::Multiple(_) => Err(validation_err_code(
                "Can't pass multiple AuthSigs",
                EC::NodeInvalidMultipleAuthSigs,
                None,
            )),
        }
    }
}

/// This struct is used both to represent various authentication material,
/// eg. wallet sigs, session sigs or cosmos auth sigs etc.
#[derive(Serialize, Clone, PartialEq, Default)]
#[cfg_attr(test, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct JsonAuthSig {
    pub sig: String,
    pub derived_via: String,
    pub signed_message: String,

    // TODO: Make this private once extract_user_address has stabilized
    pub address: String,
    pub algo: Option<String>,

    #[serde(skip)]
    auth_material_type: AuthMaterialType,

    /// The chain that the auth sig has been validated against.
    ///
    /// This is None if the auth sig has not been validated yet.
    #[serde(skip)]
    chain: Option<Chain>,
}

#[cfg(not(test))]
impl fmt::Debug for JsonAuthSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JsonAuthSig")
            .field("sig", &"****filtered****")
            .field("derived_via", &self.derived_via)
            .field("signed_message", &self.signed_message)
            .field("address", &self.address)
            .field("algo", &self.algo)
            .field("auth_material_type", &self.auth_material_type)
            .field("chain", &self.chain)
            .finish()
    }
}

#[cfg(not(test))]
impl fmt::Display for JsonAuthSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsonAuthSig {{ sig: ****filtered****, derived_via: {}, signed_message: {}, address: {}, algo: {:?}, auth_material_type: {:?}, chain: {:?} }}",
            self.derived_via, self.signed_message, self.address, self.algo, self.auth_material_type, self.chain
        )
    }
}

impl JsonAuthSig {
    pub fn new(
        sig: String,
        derived_via: String,
        signed_message: String,
        address: String,
        algo: Option<String>,
    ) -> Self {
        JsonAuthSig {
            sig,
            derived_via,
            signed_message,
            address,
            algo,
            auth_material_type: AuthMaterialType::default(),
            chain: None,
        }
    }

    /// Checks if the user address is an EVM-compatible address.
    ///
    /// NOTE: This should be used after running validation has been run on the auth sig,
    /// eg. with validate_and_get_wallet_sig.
    pub async fn is_evm_user_address(&self, bls_root_pubkey: &String) -> Result<bool> {
        if let Some(chain) = &self.chain {
            return Ok(chain::is_evm_compatible_chain(chain));
        }

        if matches!(self.auth_material_type, AuthMaterialType::WalletSig)
            || matches!(self.auth_material_type, AuthMaterialType::ContractSig)
        {
            // Try to infer the chain from the address by doing hex decode.
            let user_address = self
                .user_address(bls_root_pubkey)
                .await
                .map_err(|e| unexpected_err(e, Some("Unable to get user address".into())))?;
            let user_address = encoding::hex_to_bytes(user_address).map_err(|e| {
                conversion_err(e, Some("Error converting hex string to bytes".into()))
            });
            if user_address.is_err() {
                return Ok(false);
            }

            return Ok(true);
        } else if matches!(self.auth_material_type, AuthMaterialType::SessionSig) {
            // Extract the inner wallet sig.
            let inner_wallet_sig = extract_wallet_sig(self, bls_root_pubkey).await?;
            return Box::pin(inner_wallet_sig.is_evm_user_address(bls_root_pubkey)).await;
        }

        Ok(false)
    }

    pub fn new_with_type(
        sig: String,
        derived_via: String,
        signed_message: String,
        address: String,
        algo: Option<String>,
        auth_material_type: AuthMaterialType,
        chain: Option<Chain>,
    ) -> Self {
        JsonAuthSig {
            sig,
            derived_via,
            signed_message,
            address,
            algo,
            auth_material_type,
            chain,
        }
    }

    pub fn auth_material_type(&self) -> &AuthMaterialType {
        &self.auth_material_type
    }

    pub fn chain(&self) -> Option<&Chain> {
        self.chain.as_ref()
    }

    /// Always defaults to interpreting as a wallet sig. This is only because
    /// we don't want to break clients too much.
    ///
    /// TODO: After a stabilization period, we should make our pattern matching
    /// stricter and perhaps turn this function to returning a core::Result.
    pub(self) fn determine_auth_material_type(
        derived_via: &str,
        algo: &Option<String>,
    ) -> AuthMaterialType {
        if derived_via == AUTH_SIG_DERIVED_VIA_SESSION_SIG {
            if let Some(algo) = algo {
                if algo == AUTH_SIG_SESSION_SIG_ALGO {
                    return AuthMaterialType::SessionSig;
                }
            }
        }

        if derived_via == AUTH_SIG_DERIVED_VIA_CONTRACT_SIG
            || derived_via == AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256
        {
            return AuthMaterialType::ContractSig;
        }

        if derived_via == AUTH_SIG_DERIVED_VIA_BLS_NETWORK_SIG {
            if let Some(algo) = algo {
                if algo == AUTH_SIG_BLS_NETWORK_SIG_ALGO {
                    return AuthMaterialType::BLSNetworkSig;
                }
            }
        }

        AuthMaterialType::WalletSig
    }

    /// Returns the user address associated with the wallet sig of this auth sig object without performing ALL validation.
    pub async fn user_address(&self, bls_root_pubkey: &String) -> Result<String> {
        match self.auth_material_type {
            AuthMaterialType::WalletSig => Ok(self.address.clone()),
            AuthMaterialType::BLSNetworkSig => Ok(self.address.clone()),
            AuthMaterialType::ContractSig => Ok(self.address.clone()),
            AuthMaterialType::SessionSig => {
                Ok(extract_wallet_sig(self, bls_root_pubkey).await?.address)
            }
        }
    }

    /// Returns the wallet sig associated with this auth sig object without performing ALL validation.
    pub async fn wallet_sig(&self, bls_root_pubkey: &String) -> Result<JsonAuthSig> {
        match self.auth_material_type {
            AuthMaterialType::BLSNetworkSig => Ok(self.clone()),
            AuthMaterialType::WalletSig => Ok(self.clone()),
            AuthMaterialType::ContractSig => Ok(self.clone()),
            AuthMaterialType::SessionSig => extract_wallet_sig(self, bls_root_pubkey).await,
        }
    }

    /// Returns the wallet sig associated with this auth sig object after performing validation.
    ///
    /// Validation is done by checking capabilities for performing the requested abilities against the requested
    /// lit resources.
    pub async fn validate_and_get_wallet_sig(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        chain: &Option<String>,
        cfg: &LitConfig,
        bls_root_pubkey: &String,
        endpoint_version: &EndpointVersion,
    ) -> Result<JsonAuthSig> {
        let mut new_auth_sig = match self.auth_material_type {
            AuthMaterialType::WalletSig => {
                validate_wallet_sig_by_chain(self, chain, cfg).await?;
                self.clone()
            }
            AuthMaterialType::BLSNetworkSig => {
                return Err(validation_err_code(
                    "BLSNetworkSig is not supported for wallet sig validation",
                    EC::NodeInvalidAuthSig,
                    None,
                ));
            }
            AuthMaterialType::ContractSig => {
                let enable_siwe_validation = matches!(cfg.enable_siwe_validation(), Ok(true));
                let _ = validate_siwe_message(self, enable_siwe_validation)?;
                validate_eip1271_signature(self, chain).await?;
                self.clone()
            }
            AuthMaterialType::SessionSig => {
                // TODO: This is only here for backwards compatibility. Once the new implementation on the
                // SDK has been stabilized, we should only use validate_session_sig.
                #[allow(deprecated)]
                if *endpoint_version == EndpointVersion::Initial {
                    if let Ok(valid_auth_sig) = validate_session_sig(
                        self,
                        requested_lit_resource_ability,
                        chain,
                        cfg,
                        bls_root_pubkey,
                    )
                    .await
                    {
                        valid_auth_sig
                    } else {
                        validate_and_extract_wallet_sig(self)?
                    }
                } else {
                    validate_session_sig(
                        self,
                        requested_lit_resource_ability,
                        chain,
                        cfg,
                        bls_root_pubkey,
                    )
                    .await?
                }
            }
        };

        // Set the chain if it's present.
        if let Some(c) = chain {
            new_auth_sig.chain = Some(
                Chain::from_str(c).map_err(|e| unexpected_err("Unable to parse chain", None))?,
            );
            Ok(new_auth_sig)
        } else {
            Ok(new_auth_sig)
        }
    }

    /// Returns the resources this auth sig has requested to operate on without performing any validation.
    pub fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        match self.auth_material_type {
            AuthMaterialType::WalletSig => Ok(vec![]),
            AuthMaterialType::BLSNetworkSig => Ok(vec![]),
            AuthMaterialType::ContractSig => Ok(vec![]),
            AuthMaterialType::SessionSig => extract_requested_resources_from_session_sig(self),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultipleAuthSigs {
    pub ethereum: Option<JsonAuthSig>,
    pub solana: Option<JsonAuthSig>,
    pub cosmos: Option<JsonAuthSig>,
    pub kyve: Option<JsonAuthSig>,
    pub cheqd: Option<JsonAuthSig>,
    pub juno: Option<JsonAuthSig>,
}

impl MultipleAuthSigs {
    /// This validates each auth sig in the MultipleAuthSigs object that is not None.
    pub async fn validate_all_and_extract_wallet_sigs(
        &mut self,
        requested_lit_resource_ability: &LitResourceAbility,
        cfg: &LitConfig,
        bls_root_pubkey: &String,
        endpoint_version: &EndpointVersion,
    ) -> Result<()> {
        if self.ethereum.is_none()
            && self.solana.is_none()
            && self.cosmos.is_none()
            && self.kyve.is_none()
            && self.cheqd.is_none()
            && self.juno.is_none()
        {
            return Err(validation_err_code(
                "No auth sig detected",
                EC::NodeInvalidMultipleAuthSigs,
                None,
            )
            .add_source_to_details());
        }

        if let Some(auth_sig) = &self.ethereum {
            self.ethereum = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_ETHEREUM.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.solana {
            self.solana = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_SOLANA.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.cosmos {
            self.cosmos = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_COSMOS.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.kyve {
            self.kyve = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_KYVE.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.cheqd {
            self.cheqd = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_CHEQD.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.juno {
            self.juno = Some(
                auth_sig
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_JUNO.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        Ok(())
    }

    /// This validates each auth sig in the MultipleAuthSigs object that is not None, and returns the LAST valid auth sig.
    pub async fn validate_all_and_extract_single_wallet_sig(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        cfg: &LitConfig,
        bls_root_pubkey: &String,
        endpoint_version: &EndpointVersion,
    ) -> Result<JsonAuthSig> {
        let mut wallet_sig = None;
        if let Some(auth_sig) = &self.ethereum {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_ETHEREUM.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        if let Some(auth_sig) = &self.solana {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_SOLANA.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        if let Some(auth_sig) = &self.cosmos {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_COSMOS.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        if let Some(auth_sig) = &self.kyve {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_KYVE.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        if let Some(auth_sig) = &self.cheqd {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_CHEQD.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        if let Some(auth_sig) = &self.juno {
            auth_sig
                .validate_and_get_wallet_sig(
                    requested_lit_resource_ability,
                    &Some(CHAIN_JUNO.into()),
                    cfg,
                    bls_root_pubkey,
                    endpoint_version,
                )
                .await?;
            wallet_sig = Some(auth_sig.clone());
        }

        wallet_sig.expect_or_err_code(EC::NodeInvalidMultipleAuthSigs, "No auth sig detected")
    }

    pub fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        let mut resources = Vec::new();
        if let Some(auth_sig) = &self.ethereum {
            resources = auth_sig.resources()?;
        }

        if let Some(auth_sig) = &self.solana {
            resources = auth_sig.resources()?;
        }

        if let Some(auth_sig) = &self.cosmos {
            resources = auth_sig.resources()?;
        }

        if let Some(auth_sig) = &self.kyve {
            resources = auth_sig.resources()?;
        }

        if let Some(auth_sig) = &self.cheqd {
            resources = auth_sig.resources()?;
        }

        if let Some(auth_sig) = &self.juno {
            resources = auth_sig.resources()?;
        }

        Ok(resources)
    }

    pub fn populate_by_chain(chain: &Option<String>, auth_sig: &JsonAuthSig) -> MultipleAuthSigs {
        let mut mult_auth_sigs = MultipleAuthSigs::default();

        match chain {
            None => {
                mult_auth_sigs.ethereum = Some(auth_sig.to_owned());
            }
            Some(chain) => match chain.as_str() {
                "solana" | "solanaDevnet" | "solanaTestnet" => {
                    mult_auth_sigs.solana = Some(auth_sig.to_owned());
                }
                "cosmos" => {
                    mult_auth_sigs.cosmos = Some(auth_sig.to_owned());
                }
                "kyve" => {
                    mult_auth_sigs.kyve = Some(auth_sig.to_owned());
                }
                "cheqd" | "cheqdMainnet" | "cheqdTestnet" => {
                    mult_auth_sigs.cheqd = Some(auth_sig.to_owned());
                }
                "juno" => {
                    mult_auth_sigs.juno = Some(auth_sig.to_owned());
                }
                _ => {
                    mult_auth_sigs.ethereum = Some(auth_sig.to_owned());
                }
            },
        }

        mult_auth_sigs
    }
}

// Custom deserialization logic for JsonAuthSig

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "camelCase")]
enum JsonAuthSigField {
    Sig,
    DerivedVia,
    SignedMessage,
    Address,
    Algo,
}

impl<'de> Deserialize<'de> for JsonAuthSig {
    fn deserialize<D>(deserializer: D) -> CoreResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(JsonAuthSigVisitor)
    }
}

struct JsonAuthSigVisitor;

impl<'de> Visitor<'de> for JsonAuthSigVisitor {
    type Value = JsonAuthSig;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a map with keys sig, derivedVia, signedMessage, address, and optionally algo"
        )
    }

    fn visit_map<M>(self, mut map: M) -> CoreResult<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut sig = None;
        let mut derived_via = None;
        let mut signed_message = None;
        let mut address = None;
        let mut algo = None;

        while let Some(key) = map.next_key()? {
            match key {
                JsonAuthSigField::Sig => {
                    if sig.is_some() {
                        return Err(serde::de::Error::duplicate_field("sig"));
                    }
                    sig = Some(map.next_value()?);
                }
                JsonAuthSigField::DerivedVia => {
                    if derived_via.is_some() {
                        return Err(serde::de::Error::duplicate_field("derived_via"));
                    }
                    derived_via = Some(map.next_value()?);
                }
                JsonAuthSigField::SignedMessage => {
                    if signed_message.is_some() {
                        return Err(serde::de::Error::duplicate_field("signed_message"));
                    }
                    signed_message = Some(map.next_value()?);
                }
                JsonAuthSigField::Address => {
                    if address.is_some() {
                        return Err(serde::de::Error::duplicate_field("address"));
                    }
                    address = Some(map.next_value()?);
                }
                JsonAuthSigField::Algo => {
                    if algo.is_some() {
                        return Err(serde::de::Error::duplicate_field("algo"));
                    }
                    algo = map.next_value()?;
                }
            }
        }

        let sig: String = sig.ok_or_else(|| serde::de::Error::missing_field("sig"))?;
        let derived_via: String =
            derived_via.ok_or_else(|| serde::de::Error::missing_field("derived_via"))?;
        let signed_message: String =
            signed_message.ok_or_else(|| serde::de::Error::missing_field("signed_message"))?;
        let address: String = address.ok_or_else(|| serde::de::Error::missing_field("address"))?;

        // Determine the auth material type
        let auth_material_type = JsonAuthSig::determine_auth_material_type(&derived_via, &algo);

        Ok(JsonAuthSig::new_with_type(
            sig,
            derived_via,
            signed_message,
            address,
            algo,
            auth_material_type,
            None,
        ))
    }
}

pub fn siwe_hash_to_bls_session_hash(siwe_hash: Vec<u8>) -> Vec<u8> {
    // for BLS, we don't just sign the raw SIWE message - we add a prefix then hash again.
    // we do this to namespace our signatures since we're using the BLS network key.
    // this is just a precaution to avoid signing the wrong data on the wrong code path
    let prefixed = format!("lit_session:{}", hex::encode(siwe_hash));
    let mut hasher = Sha256::new();
    hasher.update(prefixed);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::auth::auth_material::{
        AuthMaterialType, JsonAuthSig, AUTH_SIG_DERIVED_VIA_CONTRACT_SIG,
        AUTH_SIG_DERIVED_VIA_SESSION_SIG, AUTH_SIG_SESSION_SIG_ALGO,
    };

    struct SerTestCase {
        input: JsonAuthSig,
        expected: &'static str,
    }

    struct DeserTestCase {
        input: &'static str,
        expected: JsonAuthSig,
    }

    #[test]
    fn test_ser() {
        let test_cases = get_test_ser_test_cases();

        for test_case in test_cases {
            let serialized = serde_json::to_string(&test_case.input).expect("err ser");
            assert_eq!(serialized, test_case.expected);
        }
    }

    #[test]
    fn test_deser() {
        let test_cases = get_test_deser_test_cases();

        for test_case in test_cases {
            let deserialized: JsonAuthSig =
                serde_json::from_str(test_case.input).expect("err deser");
            assert_eq!(deserialized, test_case.expected);
        }
    }

    fn get_test_ser_test_cases() -> Vec<SerTestCase> {
        vec![
            SerTestCase {
                input: JsonAuthSig::new(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                ),
                expected: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":null}"#,
            },
            SerTestCase {
                input: JsonAuthSig::new(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    Some("algo".into()),
                ),
                expected: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":"algo"}"#,
            },
        ]
    }

    fn get_test_deser_test_cases() -> Vec<DeserTestCase> {
        vec![
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address"}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::WalletSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":null}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::WalletSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"EIP1271","signedMessage":"signed_message","address":"address","algo":null}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    AUTH_SIG_DERIVED_VIA_CONTRACT_SIG.into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::ContractSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"litSessionSignViaNacl","signedMessage":"signed_message","address":"address","algo":"ed25519"}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                    "signed_message".into(),
                    "address".into(),
                    Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                    AuthMaterialType::SessionSig,
                    None,
                ),
            },
        ]
    }
}

#[cfg(test)]
mod multiple_auth_sigs_tests {
    use crate::{
        auth::auth_material::{
            AuthMaterialType, JsonAuthSig, AUTH_SIG_DERIVED_VIA_SESSION_SIG,
            AUTH_SIG_SESSION_SIG_ALGO,
        },
        models::auth::{
            LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
            LitResourcePrefix, SessionKeySignedMessage,
        },
    };

    use super::MultipleAuthSigs;

    #[derive(Debug)]
    struct TestResourcesTestCase {
        multiple_auth_sigs: MultipleAuthSigs,
        expected_resource_ids: Vec<String>,
        expected_resource_prefixes: Vec<LitResourcePrefix>,
    }

    fn get_test_resources_test_cases() -> Vec<TestResourcesTestCase> {
        let signed_message = serde_json::to_string(&SessionKeySignedMessage {
            session_key: "pub_key".into(),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251"
                        .into(),
                    resource_prefix: "lit-accesscontrolcondition".into(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![],
            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
            expiration: "2024-01-01T00:00:00Z".to_string(),
            node_address: "localhost:7470".to_string(),
        })
        .unwrap();

        vec![
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs::default(),
                expected_resource_ids: vec![],
                expected_resource_prefixes: vec![],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec![
                    "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                ],
                expected_resource_prefixes: vec![LitResourcePrefix::ACC],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessage {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessage {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessage {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessage {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessage {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
        ]
    }

    #[test]
    fn test_resources() {
        let test_cases = get_test_resources_test_cases();

        for test_case in test_cases {
            let resources = test_case.multiple_auth_sigs.resources();
            assert!(resources.is_ok());
            let resources = resources.unwrap();
            assert_eq!(resources.len(), test_case.expected_resource_ids.len());

            for (i, expected_resource_id) in test_case.expected_resource_ids.iter().enumerate() {
                assert_eq!(expected_resource_id, resources[i].get_resource_id());
                assert_eq!(
                    test_case.expected_resource_prefixes[i],
                    resources[i].get_resource_prefix()
                );
            }
        }
    }
}
