// I think I'd like this to be renamed, but I can't think of "to what" right now.
use crate::{
    error::Result, peers::peer_state::models::PeerSocketAddress, tss::common::key_type::KeyType,
};
use lit_core::config::LitConfig;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait EpochManager: Debug + Send + Sync {
    #[doc = "Change the epoch of the DKG.  This will either reshare or refresh the DKG depending on the node set, managing as many DKGs as needed to cover the new node set."]
    async fn change_epoch(
        &self,
        dkg_id: String,
        epoch: u64,
        cfg: &LitConfig,
        current_peers: &Vec<PeerSocketAddress>,
        new_peers: &Vec<PeerSocketAddress>,
    ) -> Result<(bool, Vec<String>)>; // the result a boolean success indicator and a a list of public keys that have been touched.
    fn key_type(&self) -> KeyType;
}
