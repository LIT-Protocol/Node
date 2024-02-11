use crate::constants::Chain;

// since we are always adding new EVM chains, this test actually
// looks for matches against the non EVM chains.
// generally we anticipate that the number of non EVM chains will
// grow much, much slower than EVM chains so this is the easiest solution
// compared to trying to enumerate all EVM chains.
pub fn is_evm_compatible_chain(chain: &Chain) -> bool {
    !matches!(
        chain,
        Chain::Cosmos
            | Chain::Kyve
            | Chain::Cheqd
            | Chain::CheqdMainnet
            | Chain::CheqdTestnet
            | Chain::Juno
            | Chain::Solana
            | Chain::EvmosCosmos
            | Chain::EvmosCosmosTestnet
    )
}
