use blsful::inner_types::{Bls12381G1, G1Projective};
use elliptic_curve::CurveArithmetic as CA;
use k256::Secp256k1;
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

use crate::error::{unexpected_err, Result};
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::common::storage::write_key_share_to_disk;

use crate::tss::common::restore::restorable_key_share::RestorableKeyShare;

/// Identifier for a Recovery Party member.
pub type RecPartyMemberIdType = String;
/// Decryption shares
pub type DecryptionShare<C> = verifiable_share_encryption::DecryptionShare<Vec<u8>, C>;

/// Encrypted Key Share And Decryption Shares;
pub(crate) struct EksAndDs<C: RestorableKeyShare> {
    pub public_key: C::Point,
    pub public_key_in_file_name: String,
    pub encrypted_key_share: VerifiableBackup<C>,
    /// Each recovery member can submit only one decryption share.
    /// If they submit more, the newer overwrites the older.
    pub decryption_shares: BTreeMap<RecPartyMemberIdType, DecryptionShare<C>>,
    pub restored: bool,
}

impl<C: RestorableKeyShare> EksAndDs<C> {
    pub async fn try_restore(
        &self,
        threshold: usize,
        blinder: &C::Scalar,
        epoch: u64,
        staker_address: &str,
    ) -> Option<String> {
        // If this key is already restored, return.
        if self.restored {
            return None;
        }
        // If this key does not have enough decryption shares, don't attempt.
        if self.decryption_shares.len() < threshold {
            return None;
        }

        // Decrypt the private share
        let private_share = match C::decrypt_with_shares_and_unblind(
            blinder,
            &self.get_decryption_shares(),
            &self.encrypted_key_share.ciphertext,
        ) {
            Ok(share) => share,
            Err(e) => {
                error!(
                    "Failed to decrypt share {:?} due to error: {}",
                    self.encrypted_key_share.public_key, e
                );
                return None;
            }
        };

        // Generate the key share wrapper to be kept locally
        let key_share = C::construct_local_key_share(
            &self.public_key,
            private_share,
            &self.encrypted_key_share,
        );

        // Write the key share wrapper to the disk
        let written = write_key_share_to_disk(
            &self.public_key_in_file_name,
            self.encrypted_key_share.share_index,
            epoch,
            C::curve_type(),
            staker_address,
            &key_share,
        )
        .await;

        if let Err(e) = written {
            error!(
                "Failed to save the decrypted key with public key: {} to the disk: {}",
                &self.encrypted_key_share.public_key, e,
            );
            return None;
        }
        Some(self.encrypted_key_share.public_key.clone())
    }

    // Given `pub_keys` is a subset of the pub_keys of `EksAndDs`s, in the same order,
    // this function marks all specified `EksAndDs` instances as `restored`.
    pub fn mark_keys_restored(eks_and_ds_vec: &mut Vec<EksAndDs<C>>, pub_keys: &Vec<String>) {
        for pub_key in pub_keys.iter() {
            for eks_and_ds in eks_and_ds_vec.iter_mut() {
                if &eks_and_ds.encrypted_key_share.public_key == pub_key {
                    eks_and_ds.restored = true;
                    break;
                }
            }
        }
    }

    fn get_decryption_shares(&self) -> Vec<DecryptionShare<C>> {
        self.decryption_shares
            .values()
            .map(|s| (*s).clone())
            .collect()
    }
}

impl TryFrom<VerifiableBackup<Bls12381G1>> for EksAndDs<Bls12381G1> {
    type Error = crate::error::Error;
    fn try_from(encrypted_key_share: VerifiableBackup<Bls12381G1>) -> Result<EksAndDs<Bls12381G1>> {
        // Deserialize the public key
        let public_key = G1Projective::from_uncompressed_hex(&encrypted_key_share.public_key);
        let public_key = match public_key.is_some().into() {
            true => public_key.unwrap(),
            false => {
                let err_msg = format!(
                    "Cannot deserialize BLS public key: {}",
                    encrypted_key_share.public_key
                );
                return Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None));
            }
        };

        Ok(EksAndDs {
            public_key,
            public_key_in_file_name: String::new(),
            encrypted_key_share,
            decryption_shares: BTreeMap::new(),
            restored: false,
        })
    }
}

impl TryFrom<VerifiableBackup<Secp256k1>> for EksAndDs<Secp256k1> {
    type Error = crate::error::Error;
    fn try_from(encrypted_key_share: VerifiableBackup<Secp256k1>) -> Result<EksAndDs<Secp256k1>> {
        use crate::utils::encoding::UncompressedPointHex;
        // Deserialize the public key
        let public_key = match <Secp256k1 as CA>::AffinePoint::from_uncompressed_hex(
            &encrypted_key_share.public_key,
        ) {
            Some(public_key) => public_key.into(),
            None => {
                let err_msg = format!(
                    "Cannot deserialize ECDSA public key: {}",
                    encrypted_key_share.public_key
                );
                return Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None));
            }
        };

        Ok(EksAndDs {
            public_key,
            public_key_in_file_name: String::new(),
            encrypted_key_share,
            decryption_shares: BTreeMap::new(),
            restored: false,
        })
    }
}
