use blsful::inner_types::{Bls12381G1, G1Projective};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use verifiable_share_encryption::VerifiableEncryption;

use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::dkg::gennaro::GennaroState;
use lit_core::config::LitConfig;

use super::models::BlsKeyShare;
impl GennaroState<G1Projective> {
    pub async fn generate_backup(
        encryption_key: <Bls12381G1 as BCA>::Point,
        key_share: &BlsKeyShare<G1Projective>,
        blinder: &<Bls12381G1 as BCA>::Scalar,
        cfg: &LitConfig,
    ) -> Result<VerifiableBackup<Bls12381G1>> {
        let rng = elliptic_curve::rand_core::OsRng;
        let (ciphertext, proof) = Bls12381G1::blind_encrypt_and_prove(
            encryption_key,
            &key_share.private_share,
            blinder,
            &[],
            rng,
        );

        let public_key = hex::encode(G1Projective::to_uncompressed(&key_share.public_key));

        Ok(VerifiableBackup {
            subnet_id: cfg.subnet_id()?,
            staker_address: cfg.staker_address()?,
            share_index: key_share.index,
            ciphertext,
            proof,
            txn_prefix: key_share.txn_prefix.clone(),
            public_key,
            threshold: key_share.threshold,
            total_shares: key_share.total_shares,
        })
    }
}

#[cfg(test)]
mod test {
    use blsful::inner_types::{Bls12381G1, G1Projective};
    use bulletproofs::BulletproofCurveArithmetic as BCA;
    use elliptic_curve::{Field, Group};
    use verifiable_share_encryption::{VerifiableEncryption, VerifiableEncryptionDecryptor};

    use super::VerifiableBackup;
    use crate::tests::{common::get_backup_config, key_shares::TEST_BLS_KEY_SHARE};
    use crate::tss::{blsful::models::BlsKeyShare, dkg::gennaro::GennaroState};

    fn get_enc_dec_key_pair() -> (<Bls12381G1 as BCA>::Point, <Bls12381G1 as BCA>::Scalar) {
        let mut rng = elliptic_curve::rand_core::OsRng;
        let decryption_key = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
        let encryption_key = G1Projective::generator() * decryption_key;
        (encryption_key, decryption_key)
    }

    fn recover_from_backup(
        blinder: &<Bls12381G1 as BCA>::Scalar,
        decryption_key: &<Bls12381G1 as BCA>::Scalar,
        backup: &VerifiableBackup<Bls12381G1>,
    ) -> BlsKeyShare<G1Projective> {
        let secret_share =
            Bls12381G1::decrypt_and_unblind(blinder, decryption_key, &backup.ciphertext).unwrap();
        BlsKeyShare::<G1Projective> {
            private_share: secret_share,
            public_key: G1Projective::from_uncompressed_hex(&backup.public_key).unwrap(),
            index: backup.share_index,
            threshold: backup.threshold,
            total_shares: backup.total_shares,
            txn_prefix: backup.txn_prefix.clone(),
        }
    }

    #[tokio::test]
    async fn test_bls_backup_recovery_cycle() {
        let (encryption_key, decryption_key) = get_enc_dec_key_pair();
        let key_share: BlsKeyShare<G1Projective> =
            serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let mut rng = elliptic_curve::rand_core::OsRng;
        let blinder = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
        let cfg = get_backup_config();

        // Generate verifiable backup. Generation process includes verifying the proof.
        let backup = GennaroState::<G1Projective>::generate_backup(
            encryption_key,
            &key_share,
            &blinder,
            &cfg,
        )
        .await
        .unwrap();

        // This part is to assert that the produced pair is verifiable:
        let verification_key = G1Projective::generator() * (key_share.private_share + blinder);
        Bls12381G1::verify(
            encryption_key,
            verification_key,
            &backup.ciphertext,
            &backup.proof,
            &[],
        )
        .unwrap();

        // Assert that serialization and deserialization results back in the same data.
        let backup_json = serde_json::to_string(&backup).unwrap();
        let deserialized_backup: VerifiableBackup<Bls12381G1> =
            serde_json::from_str(&backup_json).unwrap();

        assert_eq!(
            backup.share_index, deserialized_backup.share_index,
            "share indices must match"
        );
        assert_eq!(
            backup.ciphertext, deserialized_backup.ciphertext,
            "ciphertexts must match"
        );

        assert_eq!(
            backup.txn_prefix, deserialized_backup.txn_prefix,
            "txn_prefixes must match"
        );
        assert_eq!(
            backup.subnet_id, deserialized_backup.subnet_id,
            "subnet_ids must match"
        );
        assert_eq!(
            backup.public_key, deserialized_backup.public_key,
            "public_keys must match"
        );

        // Check that the decryption generates the same key share.
        let recovered_key_share =
            recover_from_backup(&blinder, &decryption_key, &deserialized_backup);
        assert_eq!(
            key_share.private_share, recovered_key_share.private_share,
            "private shares must match"
        );
    }
}
