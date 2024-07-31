use crate::error::{config_err, conversion_err, Result};
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_core::utils::binary::hex_to_bytes;

pub const CFG_KEY_BLOCKCHAIN_CHAIN_ID: &str = "blockchain.chain_id";
pub const CFG_KEY_BLOCKCHAIN_CHAIN_NAME: &str = "blockchain.chain_name";
pub const CFG_KEY_BLOCKCHAIN_WALLET: &str = "blockchain.wallet";
pub const CFG_KEY_BLOCKCHAIN_CONTRACT: &str = "blockchain.contract";

pub const CFG_SUB_KEY_DEFAULT: &str = "default";
pub const CFG_SUB_KEY_PRIVATE_KEY: &str = "private_key";

// Used only for mapping a static value (not for lookups).
pub const CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY: &str = const_str::concat!(
    CFG_KEY_BLOCKCHAIN_WALLET, ".", CFG_SUB_KEY_DEFAULT, ".", CFG_SUB_KEY_PRIVATE_KEY
);

pub trait LitBlockchainConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn blockchain_chain_id(&self) -> Result<u64>;
    fn blockchain_chain_name(&self) -> Result<String>;
    fn blockchain_wallet_private_key(&self, wallet_key: Option<&str>) -> Result<String>;
    fn blockchain_wallet_private_key_bytes(&self, wallet_key: Option<&str>) -> Result<Vec<u8>>;
    fn blockchain_contract_gas(
        &self, contract_key: Option<&str>, method_name: Option<&str>,
    ) -> Result<i64>;
    fn blockchain_contract_gas_price(
        &self, contract_key: Option<&str>, method_name: Option<&str>,
    ) -> Result<i64>;
}

impl LitBlockchainConfig for LitConfig {
    #[inline]
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults
        Ok(builder)
    }

    #[inline]
    fn blockchain_chain_id(&self) -> Result<u64> {
        u64::try_from(self.get_int(CFG_KEY_BLOCKCHAIN_CHAIN_ID)?)
            .map_err(|e| conversion_err(e, None))
    }

    #[inline]
    fn blockchain_chain_name(&self) -> Result<String> {
        self.get_checked_string(CFG_KEY_BLOCKCHAIN_CHAIN_NAME)
    }

    #[inline]
    fn blockchain_wallet_private_key(&self, wallet_key: Option<&str>) -> Result<String> {
        blockchain_wallet_string(self, wallet_key, CFG_SUB_KEY_PRIVATE_KEY)
    }

    #[inline]
    fn blockchain_wallet_private_key_bytes(&self, wallet_key: Option<&str>) -> Result<Vec<u8>> {
        hex_to_bytes(self.blockchain_wallet_private_key(wallet_key)?)
    }

    #[inline]
    fn blockchain_contract_gas(
        &self, contract_key: Option<&str>, method_name: Option<&str>,
    ) -> Result<i64> {
        blockchain_contract_int(self, contract_key, method_name, "gas")
    }

    #[inline]
    fn blockchain_contract_gas_price(
        &self, contract_key: Option<&str>, method_name: Option<&str>,
    ) -> Result<i64> {
        blockchain_contract_int(self, contract_key, method_name, "gas_price")
    }
}

#[inline]
fn blockchain_wallet_string(
    cfg: &LitConfig, wallet_key: Option<&str>, key: &str,
) -> Result<String> {
    let keys = expand_blockchain_wallet_keys(wallet_key, key);
    for key in &keys {
        if let Ok(res) = cfg.get_section_checked_string(key) {
            return Ok(res);
        }
        if let Ok(res) = cfg.get_checked_string(key) {
            return Ok(res);
        }
    }

    Err(config_err(
        format!("blockchain wallet config not found under any of the keys: [{}]", keys.join(", ")),
        None,
    ))
}

#[inline]
fn expand_blockchain_wallet_keys(wallet_key: Option<&str>, key: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if let Some(wallet_key) = wallet_key {
        res.push(format!("{CFG_KEY_BLOCKCHAIN_WALLET}.{wallet_key}.{key}"));
    }
    res.push(format!("{CFG_KEY_BLOCKCHAIN_WALLET}.{CFG_SUB_KEY_DEFAULT}.{key}"));
    res
}

#[inline]
fn blockchain_contract_int(
    cfg: &LitConfig, contract_key: Option<&str>, method_name: Option<&str>, key: &str,
) -> Result<i64> {
    let keys = expand_blockchain_contract_keys(contract_key, method_name, key);
    for key in &keys {
        if let Ok(res) = cfg.get_section_int(key) {
            return Ok(res);
        }
        if let Ok(res) = cfg.get_int(key) {
            return Ok(res);
        }
    }

    Err(config_err(
        format!(
            "blockchain contract config not found under any of the keys: [{}]",
            keys.join(", ")
        ),
        None,
    ))
}

#[inline]
fn expand_blockchain_contract_keys(
    contract_key: Option<&str>, method_name: Option<&str>, key: &str,
) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if let Some(contract_key) = contract_key {
        if let Some(method_name) = method_name {
            res.push(format!("{CFG_KEY_BLOCKCHAIN_CONTRACT}.{contract_key}.{method_name}.{key}"));
        }
        res.push(format!(
            "{CFG_KEY_BLOCKCHAIN_CONTRACT}.{contract_key}.{CFG_SUB_KEY_DEFAULT}.{key}"
        ));
    }
    res.push(format!(
        "{CFG_KEY_BLOCKCHAIN_CONTRACT}.{CFG_SUB_KEY_DEFAULT}.{CFG_SUB_KEY_DEFAULT}.{key}"
    ));
    res
}
