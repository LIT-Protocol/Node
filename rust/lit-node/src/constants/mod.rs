use std::{fmt, str::FromStr};

pub const CHAIN_ETHEREUM: &str = "ethereum";
pub const CHAIN_SOLANA: &str = "solana";
pub const CHAIN_COSMOS: &str = "cosmos";
pub const CHAIN_KYVE: &str = "kyve";
pub const CHAIN_CHEQD: &str = "cheqd";
pub const CHAIN_CHEQD_MAINNET: &str = "cheqdMainnet";
pub const CHAIN_CHEQD_TESTNET: &str = "cheqdTestnet";
pub const CHAIN_JUNO: &str = "juno";
pub const CHAIN_EVMOS: &str = "evmos";
pub const CHAIN_EVMOS_COSMOS: &str = "evmosCosmos";
pub const CHAIN_EVMOS_COSMOS_TESTNET: &str = "evmosCosmosTestnet";
pub const CHAIN_LOCALCHAIN: &str = "localchain";

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Chain {
    Ethereum,
    Solana,
    Cosmos,
    Kyve,
    Cheqd,
    CheqdMainnet,
    CheqdTestnet,
    Juno,
    Evmos,
    Localchain,
    EvmosCosmos,
    EvmosCosmosTestnet,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chain::Ethereum => write!(f, "{}", CHAIN_ETHEREUM),
            Chain::Solana => write!(f, "{}", CHAIN_SOLANA),
            Chain::Cosmos => write!(f, "{}", CHAIN_COSMOS),
            Chain::Kyve => write!(f, "{}", CHAIN_KYVE),
            Chain::Cheqd => write!(f, "{}", CHAIN_CHEQD),
            Chain::CheqdMainnet => write!(f, "{}", CHAIN_CHEQD_MAINNET),
            Chain::CheqdTestnet => write!(f, "{}", CHAIN_CHEQD_TESTNET),
            Chain::Juno => write!(f, "{}", CHAIN_JUNO),
            Chain::Evmos => write!(f, "{}", CHAIN_EVMOS),
            Chain::Localchain => write!(f, "{}", CHAIN_LOCALCHAIN),
            Chain::EvmosCosmos => write!(f, "{}", CHAIN_EVMOS_COSMOS),
            Chain::EvmosCosmosTestnet => write!(f, "{}", CHAIN_EVMOS_COSMOS_TESTNET),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChainError;

impl FromStr for Chain {
    type Err = ParseChainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CHAIN_ETHEREUM => Ok(Chain::Ethereum),
            CHAIN_SOLANA => Ok(Chain::Solana),
            CHAIN_COSMOS => Ok(Chain::Cosmos),
            CHAIN_KYVE => Ok(Chain::Kyve),
            CHAIN_CHEQD => Ok(Chain::Cheqd),
            CHAIN_CHEQD_MAINNET => Ok(Chain::CheqdMainnet),
            CHAIN_CHEQD_TESTNET => Ok(Chain::CheqdTestnet),
            CHAIN_JUNO => Ok(Chain::Juno),
            CHAIN_EVMOS => Ok(Chain::Evmos),
            CHAIN_LOCALCHAIN => Ok(Chain::Localchain),
            CHAIN_EVMOS_COSMOS => Ok(Chain::EvmosCosmos),
            CHAIN_EVMOS_COSMOS_TESTNET => Ok(Chain::EvmosCosmosTestnet),
            _ => Ok(Chain::Ethereum), // until the below todo is done, assume EVM chain
                                      // TODO: check rpc_config.yaml and return error if chain is not supported
                                      // _ => Err(ParseChainError),
        }
    }
}
