use std::collections::HashMap;
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex};

use arc_swap::ArcSwap;
use ethers::abi::Detokenize;
use ethers::contract::builders::ContractCall;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::types::H160;
use ethers::utils::keccak256;
use log::{as_serde, warn};
use once_cell::sync::Lazy;

#[allow(unused_imports)]
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;

use crate::config::LitBlockchainConfig;
use crate::contracts::allowlist::Allowlist;
use crate::contracts::backup_recovery::BackupRecovery;
use crate::contracts::contract_resolver::ContractResolver as ContractResolverContract;
use crate::contracts::host_commands::HostCommands;
use crate::contracts::lit_token::LITToken;
use crate::contracts::multisender::Multisender;
use crate::contracts::payment_delegation::PaymentDelegation;
use crate::contracts::pkp_helper::PKPHelper;
use crate::contracts::pkp_permissions::PKPPermissions;
use crate::contracts::pkpnft::PKPNFT;
use crate::contracts::pkpnft_metadata::PKPNFTMetadata;
use crate::contracts::pubkey_router::PubkeyRouter;
use crate::contracts::rate_limit_nft::RateLimitNFT;
use crate::contracts::staking::Staking;
use crate::contracts::staking_balances::StakingBalances;
use crate::contracts::{
    ALLOWLIST_CONTRACT, BACKUP_RECOVERY_CONTRACT, CONTRACT_RESOLVER_CONTRACT,
    HOST_COMMANDS_CONTRACT, LIT_TOKEN_CONTRACT, MULTI_SENDER_CONTRACT, PAYMENT_DELEGATION_CONTRACT,
    PKP_HELPER_CONTRACT, PKP_NFT_CONTRACT, PKP_NFT_METADATA_CONTRACT, PKP_PERMISSIONS_CONTRACT,
    PUB_KEY_ROUTER_CONTRACT, RATE_LIMIT_NFT_CONTRACT, RELEASE_REGISTER_CONTRACT,
    STAKING_BALANCES_CONTRACT, STAKING_CONTRACT,
};
use crate::error::{blockchain_err, conversion_err, lock_err, Result};
use crate::resolver::contract::config::SubnetConfig;
use crate::resolver::rpc::{RpcResolver, RPC_RESOLVER};
use crate::util::ether::transaction_receipt_to_serde;
use crate::ReleaseRegister;

pub mod config;

static SUBNET_RESOLVER_CACHE: Lazy<ArcSwap<SubnetResolverCache>> =
    Lazy::new(|| ArcSwap::from(Arc::new(SubnetResolverCache::new())));
static SUBNET_RESOLVER_CACHE_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));
const SUBNET_RESOLVER_CACHE_LOCK_ERR: &str = "failed to get lock on 'SUBNET_RESOLVER_CACHE_LOCK'";

static SUBNET_CONFIG_CACHE: Lazy<ArcSwap<SubnetConfigCache>> =
    Lazy::new(|| ArcSwap::from(Arc::new(SubnetConfigCache::new())));
static SUBNET_CONFIG_CACHE_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));
const SUBNET_CONFIG_CACHE_LOCK_ERR: &str = "failed to get lock on 'SUBNET_CONFIG_CACHE_LOCK'";

#[derive(Debug, Clone)]
pub struct ContractResolver {
    subnet_id: String,
    #[cfg(feature = "env-override")]
    env: LitEnv,
    wallet_key: Option<String>,
}

impl ContractResolver {
    #[cfg(not(feature = "env-override"))]
    pub fn new(subnet_id: String, wallet_key: Option<String>) -> Self {
        // Acquire the RPC Resolver to ensure the config is loaded.
        let _ = RPC_RESOLVER.load();

        Self { subnet_id, wallet_key }
    }

    #[cfg(feature = "env-override")]
    pub fn new(subnet_id: String, env: LitEnv, wallet_key: Option<String>) -> Self {
        // Acquire the RPC Resolver to ensure the config is loaded.
        let _ = RPC_RESOLVER.load();

        Self { subnet_id, env, wallet_key }
    }

    pub fn for_subnet(&self, subnet_id: String) -> Self {
        #[cfg(not(feature = "env-override"))]
        return Self { subnet_id, wallet_key: self.wallet_key.clone() };

        #[cfg(feature = "env-override")]
        Self { subnet_id, env: self.env, wallet_key: self.wallet_key.clone() }
    }

    pub fn with_wallet_key(mut self, wallet_key: String) -> Self {
        self.wallet_key = Some(wallet_key);
        self
    }

    // Resolve functions
    pub async fn resolve<K>(&self, cfg: &LitConfig, contract_key: K) -> Result<ResolvedContract>
    where
        K: AsRef<str> + Into<String> + Send,
    {
        let cache = SUBNET_RESOLVER_CACHE.load();
        if let Some(val) = cache.get(&self.subnet_id) {
            if let Some(val) = val.get(contract_key.as_ref()) {
                return Ok(val.clone());
            }
        }

        // Cache miss
        let contract_key: String = contract_key.into();

        // Resolve
        let address = Box::pin(self.resolve_address(cfg, &contract_key)).await?;

        // Store
        let contract = ResolvedContract::new(self.subnet_id.clone(), contract_key.clone(), address);

        // Cache
        SubnetResolverCache::insert(self.subnet_id.clone(), contract_key, contract.clone())?;

        Ok(contract)
    }

    async fn resolve_address<K>(&self, cfg: &LitConfig, contract_key: K) -> Result<H160>
    where
        K: AsRef<str>,
    {
        // Instantly resolve subnet address
        if STAKING_CONTRACT.eq(contract_key.as_ref()) {
            return subnet_id_to_address(&self.subnet_id);
        }

        // Attempt to resolve via config cache
        if let Some(addr) = self.resolve_address_via_config(cfg, contract_key.as_ref())? {
            return Ok(addr);
        }

        // Resolve via default methods
        match contract_key.as_ref() {
            k if CONTRACT_RESOLVER_CONTRACT.eq(k) => {
                // Special case, found in the staking contract
                Ok(self.resolve_resolver_address(cfg).await?)
            }
            _ => Ok(self.resolve_via_resolver(cfg, contract_key).await?),
        }
    }

    fn resolve_address_via_config<K>(
        &self, _cfg: &LitConfig, contract_key: K,
    ) -> Result<Option<H160>>
    where
        K: AsRef<str>,
    {
        if let Some(val) = SubnetConfigCache::get(&self.subnet_id)? {
            if let Some(val) = val.contracts().get(contract_key.as_ref()) {
                if !val.address().is_empty() {
                    match val.address().parse::<Address>() {
                        Ok(val) => {
                            return Ok(Some(val));
                        }
                        Err(err) => {
                            warn!("failed to parse address '{}' found in subnet ('{}') resolver config for contract key: {} - {:?}",
                                val.address(), &self.subnet_id, contract_key.as_ref(), err);
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    async fn resolve_resolver_address(&self, cfg: &LitConfig) -> Result<H160> {
        let staking_contract = self.staking_contract(cfg).await?;
        let chain_id = cfg.blockchain_chain_id()?;
        let chain_name = cfg.blockchain_chain_name()?;
        staking_contract.contract_resolver().call().await
            .map_err(|e| blockchain_err(
                e,
                Some(format!(
                    "failed to call contract_resolver on staking contract (subnet id: {}, contract address: {}, chain_id: {}, chain_name: {})",
                    &self.subnet_id, staking_contract.address(), chain_id, chain_name
                )),
            ))
            .and_then(|v| {
                if v.is_zero() {
                    Err(blockchain_err(
                        format!(
                            "resolver contract address not found in staking_contract (subnet id: {})",
                            &self.subnet_id
                        ),
                        None,
                    ))
                } else {
                    Ok(v)
                }
            })
    }

    async fn resolve_via_resolver<K>(&self, cfg: &LitConfig, contract_key: K) -> Result<H160>
    where
        K: AsRef<str>,
    {
        #[cfg(not(feature = "env-override"))]
        let env = *cfg.env() as u8;

        #[cfg(feature = "env-override")]
        let env = self.env as u8;

        self.resolver_contract(cfg)
            .await?
            .get_contract(keccak256(contract_key.as_ref().as_bytes()), env)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some(format!(
                        "failed to call get_contract for '{}' on resolver_contract (subnet id: {})",
                        contract_key.as_ref(),
                        &self.subnet_id
                    )),
                )
            })
            .and_then(|v| {
                if v.is_zero() {
                    Err(blockchain_err(
                        format!(
                            "contract key '{}' not found in resolver_contract (subnet id: {})",
                            contract_key.as_ref(),
                            &self.subnet_id
                        ),
                        None,
                    ))
                } else {
                    Ok(v)
                }
            })
    }

    // Accessors
    pub fn subnet_id(&self) -> &String {
        &self.subnet_id
    }

    pub fn wallet_key(&self) -> Option<&String> {
        self.wallet_key.as_ref()
    }

    pub fn wallet_key_as_str(&self) -> Option<&str> {
        self.wallet_key.as_deref()
    }

    // Contract accessors

    // Staking

    pub async fn staking_contract(&self, cfg: &LitConfig) -> Result<Staking<Provider<Http>>> {
        Staking::load(cfg, *self.resolve(cfg, STAKING_CONTRACT).await?.address())
    }

    pub async fn staking_balances_contract(
        &self, cfg: &LitConfig,
    ) -> Result<StakingBalances<Provider<Http>>> {
        StakingBalances::load(cfg, *self.resolve(cfg, STAKING_BALANCES_CONTRACT).await?.address())
    }

    pub async fn staking_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        Staking::load_with_signer(
            cfg,
            *self.resolve(cfg, STAKING_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // ContractResolver

    pub async fn resolver_contract(
        &self, cfg: &LitConfig,
    ) -> Result<ContractResolverContract<Provider<Http>>> {
        ContractResolverContract::load(
            cfg,
            *self.resolve(cfg, CONTRACT_RESOLVER_CONTRACT).await?.address(),
        )
    }

    pub async fn resolver_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<ContractResolverContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>
    {
        ContractResolverContract::load_with_signer(
            cfg,
            *self.resolve(cfg, CONTRACT_RESOLVER_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // ReleaseRegister

    pub async fn release_register_contract(
        &self, cfg: &LitConfig,
    ) -> Result<ReleaseRegister<Provider<Http>>> {
        ReleaseRegister::load(cfg, *self.resolve(cfg, RELEASE_REGISTER_CONTRACT).await?.address())
    }

    pub async fn release_register_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        ReleaseRegister::load_with_signer(
            cfg,
            *self.resolve(cfg, RELEASE_REGISTER_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // Multisender

    pub async fn multisender_contract(
        &self, cfg: &LitConfig,
    ) -> Result<Multisender<Provider<Http>>> {
        Multisender::load(cfg, *self.resolve(cfg, MULTI_SENDER_CONTRACT).await?.address())
    }

    pub async fn multisender_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<Multisender<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        Multisender::load_with_signer(
            cfg,
            *self.resolve(cfg, MULTI_SENDER_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // LITToken

    pub async fn lit_token_contract(&self, cfg: &LitConfig) -> Result<LITToken<Provider<Http>>> {
        LITToken::load(cfg, *self.resolve(cfg, LIT_TOKEN_CONTRACT).await?.address())
    }

    pub async fn lit_token_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<LITToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        LITToken::load_with_signer(
            cfg,
            *self.resolve(cfg, LIT_TOKEN_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PubkeyRouter

    pub async fn pub_key_router_contract(
        &self, cfg: &LitConfig,
    ) -> Result<PubkeyRouter<Provider<Http>>> {
        PubkeyRouter::load(cfg, *self.resolve(cfg, PUB_KEY_ROUTER_CONTRACT).await?.address())
    }

    pub async fn pub_key_router_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PubkeyRouter<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PubkeyRouter::load_with_signer(
            cfg,
            *self.resolve(cfg, PUB_KEY_ROUTER_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PKPNFT

    pub async fn pkp_nft_contract(&self, cfg: &LitConfig) -> Result<PKPNFT<Provider<Http>>> {
        PKPNFT::load(cfg, *self.resolve(cfg, PKP_NFT_CONTRACT).await?.address())
    }

    pub async fn pkp_nft_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PKPNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PKPNFT::load_with_signer(
            cfg,
            *self.resolve(cfg, PKP_NFT_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // RateLimitNFT

    pub async fn rate_limit_nft_contract(
        &self, cfg: &LitConfig,
    ) -> Result<RateLimitNFT<Provider<Http>>> {
        RateLimitNFT::load(cfg, *self.resolve(cfg, RATE_LIMIT_NFT_CONTRACT).await?.address())
    }

    pub async fn rate_limit_nft_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<RateLimitNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        RateLimitNFT::load_with_signer(
            cfg,
            *self.resolve(cfg, RATE_LIMIT_NFT_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PKPHelper

    pub async fn pkp_helper_contract(&self, cfg: &LitConfig) -> Result<PKPHelper<Provider<Http>>> {
        PKPHelper::load(cfg, *self.resolve(cfg, PKP_HELPER_CONTRACT).await?.address())
    }

    pub async fn pkp_helper_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PKPHelper<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PKPHelper::load_with_signer(
            cfg,
            *self.resolve(cfg, PKP_HELPER_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PKPPermissions

    pub async fn pkp_permissions_contract(
        &self, cfg: &LitConfig,
    ) -> Result<PKPPermissions<Provider<Http>>> {
        PKPPermissions::load(cfg, *self.resolve(cfg, PKP_PERMISSIONS_CONTRACT).await?.address())
    }

    pub async fn pkp_permissions_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PKPPermissions<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PKPPermissions::load_with_signer(
            cfg,
            *self.resolve(cfg, PKP_PERMISSIONS_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PKPNFTMetadata

    pub async fn pkp_nft_metadata_contract(
        &self, cfg: &LitConfig,
    ) -> Result<PKPNFTMetadata<Provider<Http>>> {
        PKPNFTMetadata::load(cfg, *self.resolve(cfg, PKP_NFT_METADATA_CONTRACT).await?.address())
    }

    pub async fn pkp_nft_metadata_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PKPNFTMetadata<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PKPNFTMetadata::load_with_signer(
            cfg,
            *self.resolve(cfg, PKP_NFT_METADATA_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // Allowlist

    pub async fn allowlist_contract(&self, cfg: &LitConfig) -> Result<Allowlist<Provider<Http>>> {
        Allowlist::load(cfg, *self.resolve(cfg, ALLOWLIST_CONTRACT).await?.address())
    }

    pub async fn allowlist_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        Allowlist::load_with_signer(
            cfg,
            *self.resolve(cfg, ALLOWLIST_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // BackupRecovery
    pub async fn backup_recovery_contract(
        &self, cfg: &LitConfig,
    ) -> Result<BackupRecovery<Provider<Http>>> {
        BackupRecovery::load(cfg, *self.resolve(cfg, BACKUP_RECOVERY_CONTRACT).await?.address())
    }

    pub async fn backup_recovery_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        BackupRecovery::load_with_signer(
            cfg,
            *self.resolve(cfg, BACKUP_RECOVERY_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // PaymentDelegation
    pub async fn payment_delegation_contract(
        &self, cfg: &LitConfig,
    ) -> Result<PaymentDelegation<Provider<Http>>> {
        PaymentDelegation::load(
            cfg,
            *self.resolve(cfg, PAYMENT_DELEGATION_CONTRACT).await?.address(),
        )
    }

    pub async fn payment_delegation_contract_with_signer(
        &self, cfg: &LitConfig,
    ) -> Result<PaymentDelegation<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        PaymentDelegation::load_with_signer(
            cfg,
            *self.resolve(cfg, PAYMENT_DELEGATION_CONTRACT).await?.address(),
            self.wallet_key_as_str(),
        )
    }

    // HostCommands
    pub async fn host_commands_contract(
        &self, cfg: &LitConfig,
    ) -> Result<HostCommands<Provider<Http>>> {
        HostCommands::load(cfg, *self.resolve(cfg, HOST_COMMANDS_CONTRACT).await?.address())
    }

    // Flush functions
    pub fn flush() -> Result<()> {
        SubnetResolverCache::flush()?;
        SubnetConfigCache::flush()?;
        RpcResolver::reload()?;

        Ok(())
    }

    // Prepare calls
    pub fn prepare<M, D>(
        &self, cfg: &LitConfig, mut call: ContractCall<M, D>,
    ) -> Result<ContractCall<M, D>>
    where
        M: Middleware,
        D: Detokenize,
    {
        let mut contract_key_opt: Option<String> = None;
        if let Some(address) = call.tx.to_addr() {
            if let Some(contract_key) = self.contract_key_by_address(address)? {
                contract_key_opt = Some(contract_key)
            } else {
                warn!(subnet_id = as_serde!(self.subnet_id.clone());
                    "ContractResolver::prepare(): unable to resolve contract_key for addr '{}' (should be in cache)", address);
            }
        } else {
            warn!(subnet_id = as_serde!(self.subnet_id.clone());
                "ContractResolver::prepare(): no address found in tx!");
        }
        let method_name_opt = Some(call.function.name.clone());

        if let Ok(gas) =
            cfg.blockchain_contract_gas(contract_key_opt.as_deref(), method_name_opt.as_deref())
        {
            call = call.gas(U256::from(gas as u64));
        }

        if let Ok(gas_price) = cfg
            .blockchain_contract_gas_price(contract_key_opt.as_deref(), method_name_opt.as_deref())
        {
            call = call.gas_price(U256::from(gas_price as u64));
        }

        Ok(call)
    }

    /// This method only works with a warm cache (intended to be called from 'prepare')
    fn contract_key_by_address(&self, address: &H160) -> Result<Option<String>> {
        if let Some(contract) = self.resolved_contract_by_address(address)? {
            Ok(Some(contract.key))
        } else {
            Ok(None)
        }
    }

    /// This method only works with a warm cache (intended to be called from 'prepare')
    fn resolved_contract_by_address(&self, address: &H160) -> Result<Option<ResolvedContract>> {
        let cache = SUBNET_RESOLVER_CACHE.load();
        if let Some(v) = cache.get(&self.subnet_id) {
            if let Some(v) = v.get_by_address(address) {
                return Ok(Some(v.clone()));
            }
        }

        Ok(None)
    }

    // Handy wrapper to ensure the TX has been sent.
    pub async fn must_send<M, D>(
        &self, call: ContractCall<M, D>, err_prefix: Option<&str>,
    ) -> Result<TransactionReceipt>
    where
        M: Middleware,
        D: Detokenize,
    {
        let err_prefix = err_prefix.unwrap_or("failed to send TX");

        match call
            .send()
            .await
            .map_err(|e| {
                blockchain_err(format!("{e:?}"), Some(format!("{err_prefix}: TX submit failed")))
            })?
            .await
            .map_err(|e| blockchain_err(e, Some(format!("{err_prefix}: TX failed"))))?
        {
            Some(tx_rec) => match tx_rec.status {
                Some(status) => {
                    if status == U64::from(1) {
                        return Ok(tx_rec);
                    }

                    Err(blockchain_err(
                        format!("{err_prefix}: TX failed (invalid status: {status})"),
                        None,
                    )
                    .add_field("tx_rec", transaction_receipt_to_serde(&tx_rec)))
                }
                _ => Err(blockchain_err(
                    format!("{err_prefix}: TX failed (no status in receipt)"),
                    None,
                )
                .add_field("tx_rec", transaction_receipt_to_serde(&tx_rec))),
            },
            None => Err(blockchain_err(format!("{err_prefix}: TX failed (no receipt)"), None)),
        }
    }

    /// Combination of prepare and must_send
    pub async fn send<M, D>(
        &self, cfg: &LitConfig, call: ContractCall<M, D>, err_prefix: Option<&str>,
    ) -> Result<TransactionReceipt>
    where
        M: Middleware,
        D: Detokenize,
    {
        self.must_send(self.prepare(cfg, call)?, err_prefix).await
    }
}

impl TryFrom<&LitConfig> for ContractResolver {
    type Error = crate::error::Error;

    fn try_from(cfg: &LitConfig) -> StdResult<Self, Self::Error> {
        #[cfg(not(feature = "env-override"))]
        return Ok(Self::new(cfg.subnet_id()?, None));

        #[cfg(feature = "env-override")]
        Ok(Self::new(cfg.subnet_id()?, *cfg.env(), None))
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedContract {
    subnet_id: String,
    key: String,
    address: H160,
}

impl ResolvedContract {
    pub fn new(subnet_id: String, key: String, address: H160) -> Self {
        Self { subnet_id, key, address }
    }

    pub fn subnet_id(&self) -> &String {
        &self.subnet_id
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn address(&self) -> &H160 {
        &self.address
    }
}

// Cache

#[derive(Debug, Clone)]
struct SubnetResolverCache {
    subnets: HashMap<String, SubnetResolverCacheEntry>,
}

impl SubnetResolverCache {
    pub fn new() -> Self {
        Self { subnets: HashMap::new() }
    }

    pub fn get<S>(&self, subnet_id: S) -> Option<&SubnetResolverCacheEntry>
    where
        S: AsRef<str>,
    {
        self.subnets.get(subnet_id.as_ref())
    }

    pub fn insert<S, K>(subnet_id: S, contract_key: K, contract: ResolvedContract) -> Result<()>
    where
        S: AsRef<str> + Into<String>,
        K: Into<String>,
    {
        {
            let _lock = SUBNET_RESOLVER_CACHE_LOCK.lock().map_err(|e| {
                lock_err(e.to_string(), Some(SUBNET_RESOLVER_CACHE_LOCK_ERR.into()))
            })?;

            let cache = SUBNET_RESOLVER_CACHE.load();

            let entry = match cache.get(subnet_id.as_ref()) {
                Some(v) => v.clone_and_insert(contract_key.into(), contract),
                None => {
                    let mut v = SubnetResolverCacheEntry::new();
                    v.insert(contract_key.into(), contract);
                    v
                }
            };

            let mut subnets = cache.subnets.clone();
            let _ = subnets.insert(subnet_id.into(), entry);

            SUBNET_RESOLVER_CACHE.store(Arc::new(SubnetResolverCache { subnets }));
        }

        Ok(())
    }

    pub fn flush() -> Result<()> {
        {
            let _lock = SUBNET_RESOLVER_CACHE_LOCK.lock().map_err(|e| {
                lock_err(e.to_string(), Some(SUBNET_RESOLVER_CACHE_LOCK_ERR.into()))
            })?;

            SUBNET_RESOLVER_CACHE.store(Arc::new(Self::new()))
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SubnetResolverCacheEntry {
    contracts: HashMap<String, ResolvedContract>,
    address_map: HashMap<H160, String>,
}

impl SubnetResolverCacheEntry {
    fn new() -> Self {
        Self { contracts: HashMap::new(), address_map: HashMap::new() }
    }

    pub fn get<K>(&self, contract_key: K) -> Option<&ResolvedContract>
    where
        K: AsRef<str>,
    {
        self.contracts.get(contract_key.as_ref())
    }

    pub fn get_by_address(&self, address: &H160) -> Option<&ResolvedContract> {
        if let Some(key) = self.address_map.get(address) {
            self.get(key)
        } else {
            None
        }
    }

    fn insert<K>(&mut self, contract_key: K, contract: ResolvedContract)
    where
        K: Into<String> + Clone,
    {
        self.address_map.insert(contract.address, contract_key.clone().into());
        self.contracts.insert(contract_key.into(), contract);
    }

    fn clone_and_insert<K>(&self, contract_key: K, contract: ResolvedContract) -> Self
    where
        K: Into<String> + Clone,
    {
        let mut cloned = self.clone();
        cloned.insert(contract_key, contract);
        cloned
    }
}

#[derive(Debug, Clone)]
struct SubnetConfigCache {
    subnets: HashMap<String, Option<Arc<SubnetConfig>>>,
}

impl SubnetConfigCache {
    pub fn new() -> Self {
        Self { subnets: HashMap::new() }
    }

    pub fn get<S>(subnet_id: S) -> Result<Option<Arc<SubnetConfig>>>
    where
        S: AsRef<str> + Into<String>,
    {
        let cache = SUBNET_CONFIG_CACHE.load();

        Ok(match cache._get(subnet_id.as_ref()) {
            Some(v) => {
                // We have a loaded config.
                v.clone()
            }
            None => {
                // The config has not yet been loaded.
                let entry = SubnetConfig::load(subnet_id.as_ref()).map(Arc::new);

                {
                    let _lock = SUBNET_CONFIG_CACHE_LOCK.lock().map_err(|e| {
                        lock_err(e.to_string(), Some(SUBNET_CONFIG_CACHE_LOCK_ERR.into()))
                    })?;

                    let cache = SUBNET_CONFIG_CACHE.load(); // Reload (ensure freshness).
                    let mut subnets = cache.subnets.clone();
                    let _ = subnets.insert(subnet_id.into(), entry.clone());

                    SUBNET_CONFIG_CACHE.store(Arc::new(SubnetConfigCache { subnets }));

                    entry
                }
            }
        })
    }

    fn _get<S>(&self, subnet_id: S) -> Option<&Option<Arc<SubnetConfig>>>
    where
        S: AsRef<str>,
    {
        self.subnets.get(subnet_id.as_ref())
    }

    pub fn flush() -> Result<()> {
        {
            let _lock = SUBNET_CONFIG_CACHE_LOCK
                .lock()
                .map_err(|e| lock_err(e.to_string(), Some(SUBNET_CONFIG_CACHE_LOCK_ERR.into())))?;

            SUBNET_CONFIG_CACHE.store(Arc::new(Self::new()))
        }

        Ok(())
    }
}

// Util
fn subnet_id_to_address(subnet_id: &String) -> Result<H160> {
    let subnet_id = format!("0x{subnet_id}");
    subnet_id.parse::<Address>().map_err(|e| {
        conversion_err(e, Some(format!("failed to parse subnet id ('{subnet_id}') as address")))
    })
}
