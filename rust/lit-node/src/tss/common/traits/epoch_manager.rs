use super::dkg::BasicDkg;
use crate::error::unexpected_err;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use crate::{
    error::Result,
    peers::peer_state::models::{SimplePeer, SimplePeerExt},
};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait EpochManager: BasicDkg + Debug + Send + Sync {
    #[doc = "Change the epoch of the DKG.  This will either reshare or refresh the DKG depending on the node set, managing as many DKGs as needed to cover the new node set."]

    async fn change_epoch(
        &self,
        dkg_id: String,
        current_epoch: u64,
        addr: &str,
        current_peers: &Vec<SimplePeer>,
        new_peers: &Vec<SimplePeer>,
        dkg_type: DkgType,
    ) -> Result<(bool, Vec<String>)> {
        if new_peers.is_empty() {
            return Err(unexpected_err(
                "No new peers to launch DKG with!".to_string(),
                None,
            ));
        };

        if !new_peers.contains_address(addr) {
            info!("Node is not in the new peer set - skipping DKG.");
            return Ok((true, vec![]));
        };

        // let tx_metrics = self.state.tx_metrics_manager.clone();

        info!(
            "\nDoing Epoch DKG: {} \n{} \n{}",
            dkg_id,
            current_peers.debug_addresses(),
            new_peers.debug_addresses()
        );
        debug!("dkg_type- {:?}", dkg_type);
        let curve_type = self.curve_type();
        let start = std::time::Instant::now();

        // this is the first time we are launching DKG to generate HD root keys.
        if current_peers.is_empty() {
            let hd_root_key_count = match dkg_type {
                DkgType::RecoveryParty => 1,
                // _ => cfg.ecdsa_root_pubkey_count()?,
                _ => match curve_type {
                    CurveType::BLS => 1,
                    _ => 10, // FIXME!  We'll need the correct value from on-chain contracts
                },
            };

            info!(
                "Launching {} initial DKG for {} keys!",
                curve_type, hd_root_key_count
            );

            let mut hd_root_pubkeys: Vec<String> = Vec::new();

            for i in 0..hd_root_key_count {
                let dkg_id = match hd_root_key_count {
                    1 => dkg_id.clone(),
                    _ => format!("{}_key_{}", dkg_id, i),
                };

                // #[cfg(feature = "rtmetrics")]
                // let _ = tx_metrics
                //     .send_async(NewAction(MetricAction {
                //         type_id: MetricActionType::DKG,
                //         txn_id: generate_hash(dkg_id.clone()),
                //         is_start: true,
                //         is_success: true,
                //     }))
                //     .await;

                let public_key = self
                    .keygen(&dkg_id, current_epoch, new_peers, dkg_type)
                    .await;

                // #[cfg(feature = "rtmetrics")]
                // let _ = tx_metrics
                //     .send_async(NewAction(MetricAction {
                //         type_id: MetricActionType::DKG,
                //         txn_id: generate_hash(dkg_id.clone()),
                //         is_start: false,
                //         is_success: public_key.is_ok(),
                //     }))
                //     .await;

                let public_key = public_key?;

                info!(
                    "DKG Completed - Generated key #{} as pubkey {} in {:?} ms",
                    i + 1,
                    public_key,
                    start.elapsed().as_millis()
                );

                hd_root_pubkeys.push(public_key);
            }

            info!("Generated {} HD root keys!", hd_root_pubkeys.len());
            return Ok((true, hd_root_pubkeys));
        }

        let existing_root_keys = self.root_keys().await;
        let key_count = existing_root_keys.len();

        for pubkey in existing_root_keys {
            let dkg_id = match key_count {
                1 => dkg_id.clone(),
                _ => format!("{}_{}", dkg_id, pubkey),
            };

            if current_peers == new_peers {
                info!(
                    "Launching Epoch DKG with same nodes for {} - refresh key shares for {}!",
                    curve_type, &pubkey
                );
                let r = self
                    .refresh(&pubkey, &dkg_id, current_epoch, new_peers)
                    .await?;
                info!(
                    "DKG Completed - refreshed {} key {} = {} ",
                    curve_type, &pubkey, r
                );
            } else {
                info!(
                    "Launching Epoch DKG with new nodes - resharing key shares for {}!",
                    &pubkey
                );
                let r = self
                    .reshare(&pubkey, &dkg_id, current_epoch, current_peers, new_peers)
                    .await?;
                info!(
                    "DKG Completed - reshared {} key {} = {} ",
                    curve_type, &pubkey, r
                );
            }
        }
        // don't return the reshared/refreshed keys.
        Ok((true, vec![]))
    }
}
