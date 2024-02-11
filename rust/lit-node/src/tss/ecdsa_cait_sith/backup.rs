use elliptic_curve::CurveArithmetic as CA;
use k256::Secp256k1;
use lit_core::config::LitConfig;
use verifiable_share_encryption::VerifiableEncryption;

use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::ecdsa_cait_sith::{models::EcdsaKeyShare, CsEcdsaState};
use crate::utils::encoding::UncompressedPointHex;

impl CsEcdsaState {
    pub async fn generate_backup(
        encryption_key: <Secp256k1 as CA>::ProjectivePoint,
        key_share: &EcdsaKeyShare<Secp256k1>,
        blinder: &<Secp256k1 as CA>::Scalar,
        cfg: &LitConfig,
    ) -> Result<VerifiableBackup<Secp256k1>> {
        let rng = elliptic_curve::rand_core::OsRng;
        let (ciphertext, proof) = Secp256k1::blind_encrypt_and_prove(
            encryption_key,
            &key_share.private_share,
            blinder,
            &[],
            rng,
        );

        Ok(VerifiableBackup {
            subnet_id: cfg.subnet_id()?,
            staker_address: cfg.staker_address()?,
            ciphertext,
            proof,
            txn_prefix: key_share.txn_prefix.clone(),
            public_key: key_share.public_key.to_uncompressed_hex(),
            share_index: key_share.index,
            threshold: key_share.threshold,
            total_shares: key_share.total_shares,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::tss::common::backup::VerifiableBackup;
    use crate::tss::ecdsa_cait_sith::{models::EcdsaKeyShare, CsEcdsaState};
    use elliptic_curve::{CurveArithmetic as CA, Field};
    use k256::Secp256k1;
    use verifiable_share_encryption::{VerifiableEncryption, VerifiableEncryptionDecryptor};

    use crate::tests::{common::get_backup_config, key_shares::TEST_ECDSA_KEY_SHARE};
    use crate::utils::encoding::UncompressedPointHex;

    fn get_enc_dec_key_pair() -> (
        <Secp256k1 as CA>::ProjectivePoint,
        <Secp256k1 as CA>::Scalar,
    ) {
        let mut rng = elliptic_curve::rand_core::OsRng;
        let decryption_key = <Secp256k1 as CA>::Scalar::random(&mut rng);
        let encryption_key = <Secp256k1 as CA>::ProjectivePoint::GENERATOR * decryption_key;
        (encryption_key, decryption_key)
    }

    fn recover_keyshare_from_backup(
        blinder: &<Secp256k1 as CA>::Scalar,
        decryption_key: &<Secp256k1 as CA>::Scalar,
        backup: &VerifiableBackup<Secp256k1>,
    ) -> EcdsaKeyShare<Secp256k1> {
        let private_share =
            Secp256k1::decrypt_and_unblind(blinder, decryption_key, &backup.ciphertext).unwrap();
        let public_key =
            <Secp256k1 as CA>::AffinePoint::from_uncompressed_hex(&backup.public_key).unwrap();
        EcdsaKeyShare {
            private_share,
            public_key,
            index: backup.share_index,
            threshold: backup.threshold,
            total_shares: backup.total_shares,
            txn_prefix: backup.txn_prefix.clone(),
        }
    }

    #[tokio::test]
    async fn test_ecdsa_cait_sith_backup_recovery_cycle() {
        let (encryption_key, decryption_key) = get_enc_dec_key_pair();
        let key_share = serde_json::from_str(TEST_ECDSA_KEY_SHARE).unwrap();
        let mut rng = elliptic_curve::rand_core::OsRng;
        let blinder = <Secp256k1 as CA>::Scalar::random(&mut rng);
        let cfg = get_backup_config();

        // Generate verifiable backup. Generation process includes verifying the proof.
        let backup = CsEcdsaState::generate_backup(encryption_key, &key_share, &blinder, &cfg)
            .await
            .unwrap();

        // This part is to assert that the produced pair is verifiable:
        let verification_key = <k256::Secp256k1 as CA>::ProjectivePoint::GENERATOR
            * (key_share.private_share + blinder);
        Secp256k1::verify(
            encryption_key,
            verification_key,
            &backup.ciphertext,
            &backup.proof,
            &[],
        )
        .unwrap();

        // Assert that serialization and deserialization results back in the same data.
        let backup_json = serde_json::to_string(&backup).unwrap();
        let deserialized_backup: VerifiableBackup<Secp256k1> =
            serde_json::from_str(&backup_json).unwrap();
        assert_eq!(
            backup.ciphertext, deserialized_backup.ciphertext,
            "ciphertexts must match"
        );
        assert_eq!(
            backup.public_key, deserialized_backup.public_key,
            "public keys must match"
        );
        assert_eq!(
            backup.share_index, deserialized_backup.share_index,
            "indices must match"
        );
        assert_eq!(
            backup.threshold, deserialized_backup.threshold,
            "thresholds must match"
        );
        assert_eq!(
            backup.total_shares, deserialized_backup.total_shares,
            "total shares must match"
        );

        // Check that the decryption generates the same key share.
        let recovered_key_share =
            recover_keyshare_from_backup(&blinder, &decryption_key, &deserialized_backup);
        assert_eq!(
            key_share.private_share, recovered_key_share.private_share,
            "private shares must match"
        );
    }
}
