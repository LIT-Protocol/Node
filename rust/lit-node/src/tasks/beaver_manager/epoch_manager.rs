use lit_core::config::LitConfig;

use super::models::BeaverManager;
use crate::error::Result;
use crate::peers::peer_state::models::PeerSocketAddress;
use crate::tss::common::key_type::KeyType;
use crate::tss::common::traits::epoch_manager::EpochManager;

// Experimental -> not sure if this is the correct approach?
#[async_trait::async_trait]
impl EpochManager for BeaverManager {
    #[doc = "Change the epoch of the DKG.  This will either reshare or refresh the DKG depending on the node set, managing as many DKGs as needed to cover the new node set."]
    async fn change_epoch(
        &self,
        dkg_id: String,
        epoch: u64,
        cfg: &LitConfig,
        current_peers: &Vec<PeerSocketAddress>,
        new_peers: &Vec<PeerSocketAddress>,
    ) -> Result<(bool, Vec<String>)> {
        todo!()
    }

    fn key_type(&self) -> KeyType {
        KeyType::EcdsaCaitSith
    }
}
