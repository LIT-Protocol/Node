use blsful::inner_types::Bls12381G1;
use bulletproofs::BulletproofCurveArithmetic as BCA;
use elliptic_curve::CurveArithmetic as CA;
use ethers::types::H160;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::{
    LitNodeConfig, CFG_KEY_BLS_KEY_BLINDER, CFG_KEY_ECDSA_KEY_BLINDER, CFG_KEY_ENTER_RESTORE_STATE,
};
use crate::error::{parser_err, unexpected_err, Result};
use crate::tss::common::backup::VerifiableBackup;
use crate::utils::contract::get_backup_recovery_contract_with_signer;
use crate::utils::encoding::CompressedPointHex;
use lit_blockchain::contracts::backup_recovery::BackupRecoveryErrors;
use lit_core::config::{LitConfig, CFG_RESTORE_OVERRIDE_NAME};

use crate::tss::common::restore::{
    eks_and_ds::{DecryptionShare, EksAndDs, RecPartyMemberIdType},
    restorable_key_share::RestorableKeyShare,
};

/// Keeps the state of the restore mode. Only used in case of
/// disaster recovery.
// When inner state is set, it means that the node is in RESTORE mode.
// Once the restoration is completed, the inner state is set to None.
pub struct RestoreState {
    pub bls_blinder: <Bls12381G1 as BCA>::Scalar,
    pub k256_blinder: <Secp256k1 as CA>::Scalar,
    pub actively_restoring: bool,
    state: Option<InnerState>,
}

/// Inner state kept by RestoreState.
#[allow(dead_code)]
#[derive(Default)]
struct InnerState {
    pub recovery_party_members: Vec<H160>,
    pub bls_enc_key: <Bls12381G1 as BCA>::Point,
    pub k256_enc_key: k256::ProjectivePoint,
    pub bls_eks_and_ds: Vec<EksAndDs<Bls12381G1>>,
    pub ecdsa_eks_and_ds: Vec<EksAndDs<Secp256k1>>,
    pub threshold: usize,
}

impl RestoreState {
    pub fn new(cfg: &LitConfig) -> Result<Self> {
        // Use the blinders from the config if this node is configured
        // to restore the ecnrypted key shares of an older node.
        let (actively_restoring, (bls_blinder, k256_blinder)) = match cfg.enter_restore_state() {
            Ok(true) => (
                true,
                (
                    parse_bls_blinder(&cfg.bls_key_blinder()?)?,
                    parse_ecdsa_blinder(&cfg.ecdsa_key_blinder()?)?,
                ),
            ),
            _ => (false, Self::generate_blinders()),
        };
        Ok(Self {
            bls_blinder,
            k256_blinder,
            actively_restoring,
            state: None,
        })
    }

    pub fn generate_blinders() -> (<Bls12381G1 as BCA>::Scalar, <Secp256k1 as CA>::Scalar) {
        use elliptic_curve::Field;
        let mut rng = elliptic_curve::rand_core::OsRng;
        let bls_blinder = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
        let k256_blinder = <Secp256k1 as CA>::Scalar::random(&mut rng);
        (bls_blinder, k256_blinder)
    }

    pub fn initialize(
        &mut self,
        recovery_party_members: Vec<H160>,
        bls_enc_key: <Bls12381G1 as BCA>::Point,
        k256_enc_key: k256::ProjectivePoint,
        bls_shares: Vec<(String, VerifiableBackup<Bls12381G1>)>,
        ecdsa_shares: Vec<(String, VerifiableBackup<Secp256k1>)>,
        threshold: usize,
    ) -> Result<()> {
        self.assert_actively_restoring()?;
        let mut bls_eks_and_ds = Vec::with_capacity(bls_shares.len());
        for (pub_key, share) in bls_shares.into_iter() {
            let mut share = EksAndDs::try_from(share)?;
            share.public_key_in_file_name = pub_key;
            bls_eks_and_ds.push(share);
        }
        let mut ecdsa_eks_and_ds = Vec::with_capacity(ecdsa_shares.len());
        for (pub_key, share) in ecdsa_shares.into_iter() {
            let mut share = EksAndDs::try_from(share)?;
            share.public_key_in_file_name = pub_key;
            ecdsa_eks_and_ds.push(share);
        }
        self.state = Some(InnerState {
            recovery_party_members,
            bls_enc_key,
            k256_enc_key,
            bls_eks_and_ds,
            ecdsa_eks_and_ds,
            threshold,
        });
        Ok(())
    }

    pub fn add_bls_decryption_share(
        &mut self,
        rec_party_member_id: RecPartyMemberIdType,
        root_key: String,
        decryption_share: DecryptionShare<Bls12381G1>,
        staker_address: &str,
    ) -> Result<()> {
        self.assert_actively_restoring()?;
        let vector = &mut self.get_inner_state_mut()?.bls_eks_and_ds;
        Self::add_decryption_share(vector, rec_party_member_id, root_key, decryption_share)
    }

    pub fn add_ecdsa_decryption_share(
        &mut self,
        rec_party_member_id: RecPartyMemberIdType,
        root_key: String,
        decryption_share: DecryptionShare<Secp256k1>,
    ) -> Result<()> {
        self.assert_actively_restoring()?;
        let vector = &mut self.get_inner_state_mut()?.ecdsa_eks_and_ds;
        Self::add_decryption_share(vector, rec_party_member_id, root_key, decryption_share)
    }

    /// Returns the public keys of the private shares that are restored at this attempt
    pub async fn try_restore_key_shares(
        &self,
        epoch: u64,
        staker_address: &str,
    ) -> (Vec<String>, Vec<String>) {
        let state = match self.get_inner_state() {
            Ok(state) => state,
            Err(e) => return (vec![], vec![]),
        };

        let mut restored_bls_pub_keys = vec![];
        for eks_and_ds in state.bls_eks_and_ds.iter() {
            let restore_result = eks_and_ds
                .try_restore(state.threshold, &self.bls_blinder, epoch, staker_address)
                .await;
            if let Some(public_key) = restore_result {
                restored_bls_pub_keys.push(public_key);
            };
        }

        let mut restored_ecdsa_pub_keys = vec![];
        for eks_and_ds in state.ecdsa_eks_and_ds.iter() {
            let restore_result = eks_and_ds
                .try_restore(state.threshold, &self.k256_blinder, epoch, staker_address)
                .await;
            if let Some(public_key) = restore_result {
                restored_ecdsa_pub_keys.push(public_key);
            };
        }

        // return the pub keys of the recovered root key shares.
        (restored_bls_pub_keys, restored_ecdsa_pub_keys)
    }

    pub fn mark_keys_restored(&mut self, bls_pub_keys: &Vec<String>, ecdsa_pub_keys: &Vec<String>) {
        let state = match self.get_inner_state_mut() {
            Ok(state) => state,
            Err(e) => return,
        };

        EksAndDs::mark_keys_restored(&mut state.bls_eks_and_ds, bls_pub_keys);
        EksAndDs::mark_keys_restored(&mut state.ecdsa_eks_and_ds, ecdsa_pub_keys);
    }

    pub fn are_all_keys_restored(&self) -> bool {
        let state = match self.get_inner_state() {
            Ok(state) => state,
            Err(e) => return false,
        };

        state
            .bls_eks_and_ds
            .iter()
            .all(|eks_and_ds| eks_and_ds.restored)
            && state
                .ecdsa_eks_and_ds
                .iter()
                .all(|eks_and_ds| eks_and_ds.restored)
    }

    pub fn get_recovery_party_members(&self) -> Result<&Vec<H160>> {
        Ok(&self.get_inner_state()?.recovery_party_members)
    }

    /// Clear the inner_state and regenarate the blinders.
    pub fn exit(&mut self, cfg: &LitConfig) {
        self.state = None;
        self.actively_restoring = false;
        let (bls_blinder, k256_blinder) = Self::generate_blinders();
        self.bls_blinder = bls_blinder;
        self.k256_blinder = k256_blinder;
        match clear_restore_flag(cfg) {
            Ok(f) => info!("Cleared the restore flags in the config file: {}", f),
            Err(e) => warn!("Failed to clear the restore flag: {}", e),
        }
    }

    pub fn assert_actively_restoring(&self) -> Result<()> {
        match self.actively_restoring {
            true => Ok(()),
            false => Err(unexpected_err(
                Error::new(ErrorKind::Other, "Not in RESTORE state"),
                None,
            )),
        }
    }

    // Logs the state of the restore process. Should be used in caution
    // since it logs a rather lengthy piece of information. The info is
    // formatted as json, so that it could be pretty-printed when needed.
    pub fn log(&self) {
        let log_info = RestoreStateLog::from(self);
        match serde_json::to_string(&log_info) {
            Ok(json) => info!("{}", json),
            Err(e) => info!("{:?}", log_info), // should never happen in practice.
        }
    }

    #[cfg(test)]
    pub fn get_number_of_ciphertexts(&self) -> (usize, usize) {
        match &self.state {
            Some(ref state) => (state.bls_eks_and_ds.len(), state.ecdsa_eks_and_ds.len()),
            None => (0, 0),
        }
    }

    #[cfg(test)]
    pub(crate) fn fetch_bls_backup_by_pubkey_in_filename(
        &self,
        pubkey: &str,
    ) -> Option<&VerifiableBackup<Bls12381G1>> {
        match &self.state {
            Some(ref state) => self.fetch_backup(pubkey, &state.bls_eks_and_ds),
            None => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn fetch_ecdsa_backup_by_pubkey_in_filename(
        &self,
        pubkey: &str,
    ) -> Option<&VerifiableBackup<Secp256k1>> {
        match &self.state {
            Some(ref state) => self.fetch_backup(pubkey, &state.ecdsa_eks_and_ds),
            None => None,
        }
    }

    #[cfg(test)]
    fn fetch_backup<'a, C: RestorableKeyShare>(
        &'a self,
        pubkey: &str,
        vector: &'a Vec<EksAndDs<C>>,
    ) -> Option<&'a VerifiableBackup<C>> {
        vector
            .iter()
            .filter(|x| x.public_key_in_file_name == pubkey)
            .next()
            .map(|eks_and_ds| &eks_and_ds.encrypted_key_share)
    }

    fn get_inner_state(&self) -> Result<&InnerState> {
        match &self.state {
            Some(ref state) => Ok(state),
            None => Err(Self::ciphertexts_not_set()),
        }
    }

    fn get_inner_state_mut(&mut self) -> Result<&mut InnerState> {
        match &mut self.state {
            Some(ref mut state) => Ok(state),
            None => Err(Self::ciphertexts_not_set()),
        }
    }

    fn ciphertexts_not_set() -> crate::error::Error {
        unexpected_err(
            Error::new(ErrorKind::Other, "Ciphertexts are not yet set"),
            None,
        )
    }

    fn add_decryption_share<C: RestorableKeyShare>(
        vector: &mut Vec<EksAndDs<C>>,
        rec_party_member_id: RecPartyMemberIdType,
        root_key: String,
        decryption_share: DecryptionShare<C>,
    ) -> Result<()> {
        for eks_and_ds in vector.iter_mut() {
            if root_key == eks_and_ds.encrypted_key_share.public_key {
                eks_and_ds
                    .decryption_shares
                    .insert(rec_party_member_id, decryption_share);
                return Ok(());
            }
        }
        let err_msg = format!(
            "An encrypted key share with pub_key {} does not exist.",
            root_key
        );

        Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None))
    }
}

fn clear_restore_flag(cfg: &LitConfig) -> crate::error::Result<String> {
    let mut new_config_data = HashMap::<String, String>::new();
    new_config_data.insert(
        format!("node.{}", CFG_KEY_ENTER_RESTORE_STATE),
        "false".to_string(),
    );
    new_config_data.insert(format!("node.{}", CFG_KEY_BLS_KEY_BLINDER), String::new());
    new_config_data.insert(format!("node.{}", CFG_KEY_ECDSA_KEY_BLINDER), String::new());
    cfg.save_local_config(CFG_RESTORE_OVERRIDE_NAME, &new_config_data)
}

pub async fn get_blinders(
    restore_state: &Arc<RwLock<RestoreState>>,
) -> (<Bls12381G1 as BCA>::Scalar, <Secp256k1 as CA>::Scalar) {
    let restore_state = restore_state.read().await;
    (restore_state.bls_blinder, restore_state.k256_blinder)
}

fn parse_bls_blinder(blinder_str: &str) -> Result<<Bls12381G1 as BCA>::Scalar> {
    let blinder = <Bls12381G1 as BCA>::Scalar::from_be_hex(blinder_str);
    if blinder.is_some().into() {
        #[allow(clippy::unwrap_used)]
        // `unwrap` seems to be the most suitable option for `CtOption`.
        Ok(blinder.unwrap())
    } else {
        Err(parser_err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not convert to bls key blinder:{}", blinder_str),
            ),
            None,
        ))
    }
}

fn parse_ecdsa_blinder(blinder_str: &str) -> Result<<Secp256k1 as CA>::Scalar> {
    // This is the error closure so we don't repeat it in the code.
    let error = |blinder_str| {
        parser_err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not convert to ecdsa key blinder:{}", blinder_str),
            ),
            None,
        )
    };

    let bytes = hex::decode(blinder_str).map_err(|e| error(blinder_str))?;
    let scalar_primitive = elliptic_curve::scalar::ScalarPrimitive::from_slice(&bytes)
        .map_err(|e| error(blinder_str))?;
    Ok(k256::Scalar::from(&scalar_primitive))
}

/// Used to log the state of the disaster recovery.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreStateLog {
    actively_restoring: bool,
    backups_loaded: bool,
    recovery_party_members: Vec<H160>,
    bls_enc_key: String,
    k256_enc_key: String,
    bls_shares: Vec<RootKeyRecoveryLog>,
    ecdsa_shares: Vec<RootKeyRecoveryLog>,
    threshold: usize,
}

/// Encrypted Key Shares And Recovery Members ...
// who have sent decryption shares for this key
#[derive(Debug, Serialize, Deserialize)]
struct RootKeyRecoveryLog {
    public_key: String,
    restored: bool,
    members_who_sent_dec_shares: Vec<String>,
}

impl<C: RestorableKeyShare> From<&EksAndDs<C>> for RootKeyRecoveryLog {
    fn from(eks_and_ds: &EksAndDs<C>) -> Self {
        Self {
            public_key: eks_and_ds.public_key_in_file_name.clone(),
            restored: eks_and_ds.restored,
            members_who_sent_dec_shares: eks_and_ds.decryption_shares.keys().cloned().collect(),
        }
    }
}

impl From<&RestoreState> for RestoreStateLog {
    fn from(restore_state: &RestoreState) -> Self {
        match restore_state.get_inner_state() {
            Ok(state) => Self {
                actively_restoring: restore_state.actively_restoring,
                backups_loaded: true,
                recovery_party_members: state.recovery_party_members.clone(),
                bls_enc_key: state.bls_enc_key.to_compressed_hex().to_string(),
                k256_enc_key: state.k256_enc_key.to_compressed_hex().to_string(),
                bls_shares: state.bls_eks_and_ds.iter().map(|s| s.into()).collect(),
                ecdsa_shares: state.ecdsa_eks_and_ds.iter().map(|s| s.into()).collect(),
                threshold: state.threshold,
            },
            Err(_) => Self {
                actively_restoring: restore_state.actively_restoring,
                backups_loaded: false,
                recovery_party_members: Default::default(),
                bls_enc_key: Default::default(),
                k256_enc_key: Default::default(),
                bls_shares: Default::default(),
                ecdsa_shares: Default::default(),
                threshold: 0,
            },
        }
    }
}

pub enum NodeRecoveryStatus {
    Null,
    StartedInRestoreState,
    BackupsAreLoaded,
    AllKeysAreRestored,
    AbandonedRecoveryDueToNetworkState,
}

pub async fn report_progress(cfg: &LitConfig, status: NodeRecoveryStatus) {
    let recovery_contract = match get_backup_recovery_contract_with_signer(cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            warn!("RestoreState: failed to report progress: {}", e);
            return;
        }
    };

    match recovery_contract
        .set_node_recovery_status(status as u8)
        .send()
        .await
    {
        Ok(_) => info!("RestoreState: reported progress to recovery contract"),
        Err(e) => {
            warn!("RestoreState: failed to report progress: {}", e);
            if let Some(b) = e.decode_contract_revert::<BackupRecoveryErrors>() {
                warn!("RestoreState: revert data: {:?}", b);
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::config::LitNodeConfig;
    use crate::tss::common::restore::restore_state::{
        clear_restore_flag, parse_bls_blinder, parse_ecdsa_blinder, RestoreState,
    };
    use crate::utils::encoding::BeBytes;
    use lit_core::config::{LitConfig, LitConfigBuilder};

    #[tokio::test]
    async fn test_generate_serialize_and_deserialize_blinders() {
        let (bls_blinder, ecdsa_blinder) = RestoreState::generate_blinders();
        let bls_blinder_str = hex::encode(bls_blinder.to_be_bytes());
        let ecdsa_blinder_str = hex::encode(ecdsa_blinder.to_be_bytes());
        let restored_bls_blinder = parse_bls_blinder(&bls_blinder_str).unwrap();
        let restored_ecdsa_blinder = parse_ecdsa_blinder(&ecdsa_blinder_str).unwrap();
        assert_eq!(bls_blinder, restored_bls_blinder);
        assert_eq!(ecdsa_blinder, restored_ecdsa_blinder);
    }

    #[test]
    fn test_clear_restore_flag() {
        let config_builder = LitConfigBuilder::default()
            .set_default("node.enter_restore_state", true)
            .set_default("lit.env", "dev");
        let cfg = LitConfig::from_builder(config_builder).unwrap();
        if let Err(e) = clear_restore_flag(&cfg) {
            panic!("Cannot clear restore flag: {}", e);
        }
    }
}
