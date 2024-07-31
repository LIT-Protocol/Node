use blsful::inner_types::{Bls12381G1, G1Projective};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use verifiable_share_encryption::VerifiableEncryption;

use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::tss::common::backup::VerifiableBackup;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::key_share_helper::KeyHelper;
use crate::tss::common::traits::key_persistence::KeyPersistence;
use crate::tss::dkg::gennaro::GennaroMpcDkg;
use lit_core::config::LitConfig;

impl GennaroMpcDkg<G1Projective> {
    pub async fn generate_backup(
        encryption_key: <Bls12381G1 as BCA>::Point,
        disk_share: &KeyShare,
        blinder: &<Bls12381G1 as BCA>::Scalar,
        cfg: &LitConfig,
    ) -> Result<VerifiableBackup<Bls12381G1>> {
        let rng = elliptic_curve::rand_core::OsRng;

        let key_helper = KeyHelper::<G1Projective>::default();
        let key_share = &key_helper.secret_from_hex(&disk_share.hex_private_share)?;

        let (ciphertext, proof) =
            Bls12381G1::blind_encrypt_and_prove(encryption_key, key_share, blinder, &[], rng);

        let public_key = key_helper.pk_from_hex(&disk_share.hex_public_key)?;
        let public_key = hex::encode(G1Projective::to_uncompressed(&public_key));

        Ok(VerifiableBackup {
            subnet_id: cfg.subnet_id()?,
            staker_address: cfg.staker_address()?,
            share_index: disk_share.index,
            ciphertext,
            proof,
            txn_prefix: disk_share.txn_prefix.clone(),
            public_key,
            threshold: disk_share.threshold,
            total_shares: disk_share.total_shares,
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
    use crate::tss::common::curve_type::CurveType;
    use crate::tss::common::key_share::KeyShare;
    use crate::tss::common::key_share_helper::KeyHelper;
    use crate::tss::common::traits::key_persistence::KeyPersistence;
    use crate::tss::dkg::gennaro::GennaroMpcDkg;

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
    ) -> KeyShare {
        let secret_share =
            Bls12381G1::decrypt_and_unblind(blinder, decryption_key, &backup.ciphertext).unwrap();

        let key_helper = KeyHelper::<G1Projective>::default();
        let private_share = key_helper.secret_to_hex(&secret_share);
        let public_key =
            key_helper.pk_to_hex(&G1Projective::from_uncompressed_hex(&backup.public_key).unwrap());
        KeyShare {
            hex_private_share: private_share,
            hex_public_key: public_key,
            curve_type: CurveType::BLS.try_into().unwrap(),
            index: backup.share_index,
            threshold: backup.threshold,
            total_shares: backup.total_shares,
            txn_prefix: backup.txn_prefix.clone(),
        }
    }

    #[tokio::test]
    async fn test_bls_backup_recovery_cycle() {
        let (encryption_key, decryption_key) = get_enc_dec_key_pair();
        let key_share: KeyShare = serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let mut rng = elliptic_curve::rand_core::OsRng;
        let blinder = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
        let cfg = get_backup_config();

        // Generate verifiable backup. Generation process includes verifying the proof.
        let backup = GennaroMpcDkg::<G1Projective>::generate_backup(
            encryption_key,
            &key_share,
            &blinder,
            &cfg,
        )
        .await
        .unwrap();

        // This part is to assert that the produced pair is verifiable:
        let key_helper = KeyHelper::<G1Projective>::default();
        let private_share = key_helper
            .secret_from_hex(&key_share.hex_private_share)
            .unwrap();
        let verification_key = G1Projective::generator() * (private_share + blinder);
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
            key_share.hex_private_share, recovered_key_share.hex_private_share,
            "private shares must match"
        );
    }
}
