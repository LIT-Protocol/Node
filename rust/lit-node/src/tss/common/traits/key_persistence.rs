use crate::error::{unexpected_err, Result};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerExt};
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::storage::{
    read_key_share_from_disk, write_backup_to_disk, write_key_share_to_disk,
};
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};
use elliptic_curve::group::{Group, GroupEncoding};
use std::fmt::Debug;
#[async_trait::async_trait]
pub trait KeyPersistence<G>: Debug + Send + Sync
where
    G: Group + GroupEncoding + Default,
{
    fn curve_type(&self) -> CurveType;

    fn secret_to_hex(&self, share: &G::Scalar) -> String;
    fn secret_from_hex(&self, private_key: &str) -> Result<G::Scalar>;
    fn secret_from_bytes(&self, private_key: &[u8]) -> Result<G::Scalar>;

    fn pk_to_hex(&self, public_key: &G) -> String;
    fn pk_from_hex(&self, public_key: &str) -> Result<G>;
    fn pk_from_bytes(&self, public_key: &[u8]) -> Result<G>;

    fn validate_pk_len(&self, public_key: &[u8]) -> Result<()> {
        if public_key.len() != self.curve_type().compressed_point_len() {
            return Err(unexpected_err(
                "Invalid compressed public key length",
                Some(format!(
                    "Expected {} length: {}, actual length: {}",
                    self.curve_type(),
                    self.curve_type().compressed_point_len(),
                    public_key.len()
                )),
            ));
        }
        Ok(())
    }
    fn validate_scalar_len(&self, private_key: &[u8]) -> Result<()> {
        if private_key.len() != self.curve_type().scalar_len() {
            return Err(unexpected_err(
                "Invalid private key length",
                Some(format!(
                    "Expected {} length: {}, actual length: {}",
                    self.curve_type(),
                    self.curve_type().scalar_len(),
                    private_key.len()
                )),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn write_key(
        &self,
        pubkey: Option<String>,
        pk: G,
        share: G::Scalar,
        index: u16,
        dkg_id: &str,
        epoch: u64,
        peers: &Vec<SimplePeer>,
        dkg_type: DkgType,
        curve_type: CurveType,
        staker_address: &str,
    ) -> Result<String> {
        let total_shares = peers.len() as u16;
        let threshold = peers.threshold_for_set();

        let key_share = KeyShare::new(
            share,
            pk,
            curve_type,
            index,
            threshold,
            total_shares,
            dkg_id.to_string(),
        )?;

        // because refreshing return 0x0 as a result, we need to check for the exisitence of a passed public key
        // and use this value as the PK parameter and file name.
        let hex_pubkey = match pubkey {
            Some(pubkey) => pubkey,
            None => key_share.hex_public_key.clone(),
        };

        let epoch = match dkg_type {
            DkgType::Standard => epoch,
            DkgType::RecoveryParty => 0,
        };

        let write_result = write_key_share_to_disk::<KeyShare>(
            &key_share.hex_public_key,
            index,
            epoch,
            self.curve_type(),
            staker_address,
            &key_share,
        )
        .await?;

        if dkg_type.is_to_be_backed_up() {
            let backup_result = write_backup_to_disk::<KeyShare>(
                &hex_pubkey,
                index,
                self.curve_type(),
                &key_share,
                peers,
                staker_address,
            )
            .await;
            // Do not fail if backup fails, rather report it:
            if let Err(e) = backup_result {
                error!(
                    "Writing the backup data for {} pubkey: {} to the disk failed: {}",
                    self.curve_type(),
                    &hex_pubkey,
                    e
                );
            }
        }

        Ok(hex_pubkey)
    }

    async fn read_key(
        &self,
        pubkey: &str,
        share_index: u16,
        epoch: u64,
        staker_address: &str,
    ) -> Result<Option<(G::Scalar, G)>> {
        let curve_type = self.curve_type();

        let key_share = read_key_share_from_disk::<KeyShare>(
            pubkey,
            share_index,
            epoch,
            curve_type,
            staker_address,
        )
        .await?;

        let secret_share = self.secret_from_hex(&key_share.hex_private_share)?;
        let public_key = self.pk_from_hex(&key_share.hex_public_key)?;

        Ok(Some((secret_share, public_key)))
    }
}
