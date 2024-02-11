use crate::error::Result;
use crate::peers::peer_state::models::{PeerSocketAddress, PeerSocketAddressVec};
use crate::tss::common::key_type::DkgType;
use crate::tss::common::web::models::NodeConfig;
use lit_core::config::LitConfig;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait DkgSupport: Debug + Send + Sync {
    fn addr(&self) -> String;
    async fn peer_socket_addresses(&self) -> Vec<PeerSocketAddress>;
    fn lit_config(&self) -> Arc<LitConfig>;

    async fn root_keys(&self) -> Vec<String>;

    async fn keygen(
        &self,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        dkg_type: DkgType,
    ) -> Result<String>;

    async fn reshare(
        &self,
        pubkey: &str,
        dkg_id: &str,
        current_epoch: u64,
        peers: &Vec<PeerSocketAddress>,
        next_peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool>;

    async fn refresh(
        &self,
        pubkey: &str,
        dkg_id: &str,
        current_epoch: u64,
        peers: &Vec<PeerSocketAddress>,
    ) -> Result<bool>;

    fn node_config_for_dkg(
        &self,
        peers: &Vec<PeerSocketAddress>,
        txn_prefix: &str,
    ) -> Result<NodeConfig> {
        self.node_config_for_dkg_with_options(peers, txn_prefix, true)
    }

    fn node_config_for_dkg_with_options(
        &self,
        peers: &Vec<PeerSocketAddress>,
        txn_prefix: &str,
        is_one_based: bool,
    ) -> Result<NodeConfig> {
        let addr = &self.addr();

        trace!(
            "Preparing configuration for node {}, connected to {} peers {:?}.",
            addr,
            &peers.all_addresses().len(),
            &peers.all_addresses()
        );

        let total_shares = peers.all_addresses().len() as u16;
        let threshold = crate::utils::consensus::get_threshold_count(total_shares as usize) as u16;

        let share_index = peers.index_of(addr)?;

        let share_index = match is_one_based {
            true => share_index as u16 + 1,
            false => share_index as u16,
        };

        let peer_indexes: Vec<u16> = match is_one_based {
            true => (1..(total_shares + 1)).collect(),
            false => (0..total_shares).collect(),
        };

        let node_config = NodeConfig {
            addr: addr.clone(),
            current_index: share_index,
            share_index,
            txn_prefix: txn_prefix.to_string(),
            peers: peers.all_addresses(),
            peer_indexes,
            lit_config: debug_ignore::DebugIgnore(self.lit_config()),
            threshold,
            total_shares,
        };

        Ok(node_config)
    }
}
