use blsful::inner_types::Bls12381G1;
use bulletproofs::BulletproofCurveArithmetic as BCA;
use elliptic_curve::CurveArithmetic as CA;
use ethers::types::H160;
use k256::Secp256k1;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::LitNodeConfig;
use crate::error::{parser_err, unexpected_err, Result};
use crate::tss::common::backup::VerifiableBackup;
use lit_core::config::LitConfig;

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
    pub async fn try_restore_key_shares(&self, epoch: u64) -> (Vec<String>, Vec<String>) {
        let state = match self.get_inner_state() {
            Ok(state) => state,
            Err(e) => return (vec![], vec![]),
        };

        let mut restored_bls_pub_keys = vec![];
        for eks_and_ds in state.bls_eks_and_ds.iter() {
            let restore_result = eks_and_ds
                .try_restore(state.threshold, &self.bls_blinder, epoch)
                .await;
            if let Some(public_key) = restore_result {
                restored_bls_pub_keys.push(public_key);
            };
        }

        let mut restored_ecdsa_pub_keys = vec![];
        for eks_and_ds in state.ecdsa_eks_and_ds.iter() {
            let restore_result = eks_and_ds
                .try_restore(state.threshold, &self.k256_blinder, epoch)
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
    pub fn exit(&mut self) {
        self.state = None;
        self.actively_restoring = false;
        let (bls_blinder, k256_blinder) = Self::generate_blinders();
        self.bls_blinder = bls_blinder;
        self.k256_blinder = k256_blinder;
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

    pub fn generate_blinders() -> (<Bls12381G1 as BCA>::Scalar, <Secp256k1 as CA>::Scalar) {
        use elliptic_curve::Field;
        let mut rng = elliptic_curve::rand_core::OsRng;
        let bls_blinder = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
        let k256_blinder = <Secp256k1 as CA>::Scalar::random(&mut rng);
        (bls_blinder, k256_blinder)
    }
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

#[cfg(test)]
mod test {
    use crate::tss::common::restore::restore_state::{
        parse_bls_blinder, parse_ecdsa_blinder, RestoreState,
    };
    use crate::utils::encoding::BeBytes;

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
}
