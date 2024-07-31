use crate::auth::auth_material::JsonAuthSig;
use crate::config::{encrypted_key_path, LitNodeConfig};
use crate::tss::common::backup::{RecoveryParty, VerifiableBackup};
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::storage::{
    read_all_backup_from_disk, read_encrypted_keys_from_disk, read_from_disk, write_to_disk,
};
use crate::tss::dkg::gennaro::GennaroMpcDkg;
use crate::tss::ecdsa_cait_sith::CsEcdsaState;
use async_std::fs;
use async_std::path::Path;
use blsful::inner_types::{Bls12381G1, G1Projective};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use chrono::{DateTime, Utc};
#[cfg(any(feature = "testing", test))]
use elliptic_curve::Field;
use elliptic_curve::{CurveArithmetic as CA, Group};
use k256::Secp256k1;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use std::process::Stdio;
use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use tracing::trace;

use crate::endpoints::auth_sig::{check_auth_sig, LITNODE_ADMIN_RES};
use crate::error::{io_err, io_err_code, Result, EC};

#[cfg(any(feature = "testing", test))]
use ethers::types::H160;
#[cfg(any(feature = "testing", test))]
use verifiable_share_encryption::KeyToPoint;

pub(crate) fn check_admin_auth_sig(config: &LitConfig, auth_sig: &JsonAuthSig) -> Result<()> {
    let admin_address = config.admin_address()?;
    check_auth_sig(config, auth_sig, LITNODE_ADMIN_RES, &vec![admin_address])
}

// File names in tar'ed backup directory
const K256_ENCRYPTION_KEY_FN: &str = "k256_encryption_key";
const BLS_ENCRYPTION_KEY_FN: &str = "bls_encryption_key";
const RECOVERY_PARTY_WALLET_ADDRESSES_FN: &str = "recovery_party_wallet_addresses";
const SESSION_ID_FN: &str = "session_id";
const BLS_BLINDER_COMMITMENT_FN: &str = "bls_blinder_commitment";
const K256_BLINDER_COMMITMENT_FN: &str = "k256_blinder_commitment";
const THRESHOLD_FN: &str = "threshold";
const VERSION_NO_FN: &str = "version_no";
const VERSION_NO: u8 = 1;

pub(crate) async fn encrypt_and_tar_backup_keys(
    cfg: &LitConfig,
    bls_blinder: &<Bls12381G1 as BCA>::Scalar,
    k256_blinder: &<Secp256k1 as CA>::Scalar,
    recovery_party: &RecoveryParty,
    node_set_hash: Option<String>,
) -> Result<Child> {
    trace!("Encrypting and tar'ing backup keys");
    let now: DateTime<Utc> = Utc::now();

    let staker_address = &crate::endpoints::recovery::get_staker_address(cfg)?;

    // Create the temporary dir in which we will save the resulting artefacts.
    let mut path = encrypted_key_path(staker_address);
    let _ = std::fs::remove_dir_all(path.clone());
    path.push(format!("backup-{}/", now));
    fs::create_dir_all(&path)
        .await
        .map_err(|e| io_err(e, None))?;
    trace!("Created backup directory {:?}", path);

    // Add a version no.
    write_to_disk(path.clone(), VERSION_NO_FN, &VERSION_NO).await?;

    // Get recovery party information and save them in the folder.
    write_to_disk(path.clone(), SESSION_ID_FN, &recovery_party.session_id).await?;
    write_to_disk(path.clone(), THRESHOLD_FN, &recovery_party.threshold).await?;
    let k256_enc_key = &recovery_party.ecdsa_encryption_key;
    let k256_affine_point = k256::AffinePoint::from(k256_enc_key);
    write_to_disk(path.clone(), K256_ENCRYPTION_KEY_FN, &k256_affine_point).await?;
    let bls_enc_key = &recovery_party.bls_encryption_key;
    write_to_disk(path.clone(), BLS_ENCRYPTION_KEY_FN, bls_enc_key).await?;
    write_to_disk(
        path.clone(),
        RECOVERY_PARTY_WALLET_ADDRESSES_FN,
        &recovery_party.party_members,
    )
    .await?;
    trace!(
        "Recovery party wallet addresses: {:?}",
        recovery_party.party_members
    );

    // Generate the blinder commitments
    let bls_commitment = G1Projective::generator() * bls_blinder;
    let k256_commitment = <Secp256k1 as CA>::ProjectivePoint::GENERATOR * k256_blinder;
    let k256_as_affine_p = k256::AffinePoint::from(&k256_commitment);
    write_to_disk(path.clone(), BLS_BLINDER_COMMITMENT_FN, &bls_commitment).await?;
    write_to_disk(path.clone(), K256_BLINDER_COMMITMENT_FN, &k256_as_affine_p).await?;
    trace!("Blinder commitments generated");

    // Encrypt and save ECDSA keys
    let ecdsa_shares =
        read_all_backup_from_disk::<KeyShare>(&node_set_hash, CurveType::K256, staker_address)
            .await?;
    for (file_name, share) in ecdsa_shares {
        let backup =
            CsEcdsaState::generate_backup(*k256_enc_key, &share, k256_blinder, cfg).await?;
        write_to_disk(path.clone(), &file_name, &backup).await?;
    }
    trace!("ECDSA keys encrypted and saved");

    // Encrypt and save BLS keys
    let bls_shares =
        read_all_backup_from_disk::<KeyShare>(&node_set_hash, CurveType::BLS, staker_address)
            .await?;
    for (file_name, share) in bls_shares {
        let backup = GennaroMpcDkg::generate_backup(*bls_enc_key, &share, bls_blinder, cfg).await?;
        write_to_disk(path.clone(), &file_name, &backup).await?;
    }
    trace!("BLS keys encrypted and saved");

    // zip up the newly created backup directory
    // tar -czf - <path> ...
    let mut tar_cmd = Command::new("tar");
    tar_cmd
        .arg("-czf")
        .arg("-") // do not write to file, going to pipe
        .arg("-C")
        .arg(&path)
        .arg(".");

    let tar_child = spawn_child_and_check(
        tar_cmd
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()),
        "unable to tar the backup",
    )
    .await?;
    trace!("Tar'ed backup to {:?}", path);

    Ok(tar_child)
}

pub(crate) async fn untar_keys_stream<R: AsyncRead + Unpin>(
    cfg: &LitConfig,
    restore_state: &RwLock<RestoreState>,
    stream: R,
) -> Result<()> {
    {
        let state = restore_state.read().await;
        state.assert_actively_restoring()?;
    }

    let staker_address = &crate::endpoints::recovery::get_staker_address(cfg)?;

    // Create the temporary dir in which we will save the artefacts.
    let now: DateTime<Utc> = Utc::now();
    let mut path = encrypted_key_path(staker_address);
    path.push(format!("restore-{}/", now));

    // Untar the data
    untar_stream_to_path(path.as_path(), stream).await?;
    trace!("Untarred backup to {:?}", path);
    // Read the encryption keys from the folder.
    let bls_enc_key: <Bls12381G1 as BCA>::Point =
        read_from_disk(path.clone(), BLS_ENCRYPTION_KEY_FN).await?;
    trace!("BLS encryption key retrieved");
    let k256_affine_point: k256::AffinePoint =
        read_from_disk(path.clone(), K256_ENCRYPTION_KEY_FN).await?;
    let k256_enc_key = k256::ProjectivePoint::from(&k256_affine_point);
    trace!("k256 encryption key retrieved");

    let recovery_party_wallet_addresses =
        read_from_disk(path.clone(), RECOVERY_PARTY_WALLET_ADDRESSES_FN).await?;
    trace!(
        "Recovery party wallet addresses: {:?}",
        recovery_party_wallet_addresses
    );
    let threshold = read_from_disk(path.clone(), THRESHOLD_FN).await?;
    trace!("Threshold: {:?}", threshold);

    // Read BLS keys
    let encrypted_bls_shares: Vec<(String, VerifiableBackup<Bls12381G1>)> =
        read_encrypted_keys_from_disk(&path, CurveType::BLS).await?;
    trace!("BLS shares retrieved");

    // Read ECDSA keys
    let encrypted_ecdsa_shares: Vec<(String, VerifiableBackup<Secp256k1>)> =
        read_encrypted_keys_from_disk(&path, CurveType::K256).await?;
    trace!("ECDSA shares retrieved");

    let mut restore_state = restore_state.write().await;
    restore_state.initialize(
        recovery_party_wallet_addresses,
        bls_enc_key,
        k256_enc_key,
        encrypted_bls_shares,
        encrypted_ecdsa_shares,
        threshold,
    )?;

    let _ = std::fs::remove_dir_all(path);

    Ok(())
}

async fn untar_stream_to_path<R: AsyncRead + Unpin>(path: &Path, mut stream: R) -> Result<()> {
    fs::create_dir_all(&path)
        .await
        .map_err(|e| io_err(e, None))?;

    // unzip
    let mut cmd = Command::new("tar");
    cmd.arg("-xzf")
        .arg("-")
        .arg("--strip-components=1") // remove the top directory while unpacking
        .arg("-C") // --directory
        .arg(path.to_str().expect_or_err("Failed to stringify path")?)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd
        .spawn()
        .map_err(|e| io_err(e, Some("failed to spawn untar cmd".into())))?;

    let mut stdin = child
        .stdin
        .take()
        .expect_or_err("expected to have stdin for child process")?;

    let written = tokio::io::copy(&mut stream, &mut stdin)
        .await
        .map_err(|e| io_err(e, Some("failed to pipe stdin to tar cmd".into())))?;

    stdin.flush().await.map_err(|e| io_err(e, None))?;
    drop(stdin);

    let status = child
        .wait()
        .await
        .map_err(|e| io_err(e, Some("failed to wait for tar child process".into())))?;
    if !status.success() {
        return Err(io_err("untar process did not complete successfully", None));
    }

    Ok(())
}

pub(crate) async fn purge_precomputes(cfg: &LitConfig) -> Result<()> {
    // Pattern to match: "./node_precomputes/**/*-{node_name}-*.cbor"
    delete_file(format!(
        "./node_precomputes/**/*-{}-*.cbor",
        cfg.external_addr()
            .expect_or_err("expected external_addr to be set")?
            .replace(':', "-")
    ))
    .await
    .map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some("Unable to remove precomputes".into()),
        )
    })?;

    Ok(())
}

pub(crate) async fn delete_file<P: AsRef<str>>(file_path_or_pattern: P) -> Result<()> {
    Command::new("rm")
        .arg("-f")
        .arg(file_path_or_pattern.as_ref())
        .output()
        .await
        .map_err(|e| io_err(e, None))?;

    Ok(())
}

/// Guard to wait up to 50ms for the process to start so we can ensure it is running.
/// As we are streaming, we never check the exit status so this at least tries to
/// ensure that the command didn't insta-fail.
pub async fn spawn_child_and_check(cmd: &mut Command, label: &str) -> Result<Child> {
    let mut child = cmd.spawn().map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("{}: failed to spawn process", label)),
        )
    })?;

    tokio::select! {
        res = child.wait() => {
            match res {
                Ok(status) => {
                    if !status.success() {
                        return Err(io_err_code(
                            format!(
                                "{}: failed to run for process (status: {:?})",
                                label,
                                status.code()
                            ),
                            EC::NodeSystemFault,
                            None,
                        ));
                    }
                }
                Err(e) => return Err(
                    io_err_code(
                        e,
                        EC::NodeSystemFault,
                        Some(format!("{}: failed to wait on process", label)),
                    )
                )
            }
        }
        _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
            // Continue.
        }
    }

    Ok(child)
}

#[cfg(any(feature = "testing", test))]
pub fn get_test_recovery_party() -> RecoveryParty {
    // Generate mock keys
    let mut rng = elliptic_curve::rand_core::OsRng;
    let bls_decryption_key = <Bls12381G1 as BCA>::Scalar::random(&mut rng);
    let bls_encryption_key = <Bls12381G1 as BCA>::Point::generator() * bls_decryption_key;
    let ecdsa_decryption_key = bulletproofs::k256::SecretKey::random(&mut rng);
    let ecdsa_encryption_key = ecdsa_decryption_key.public_key().key_to_point();

    // Mock recovery party members
    let mut party_members = vec![];
    for _ in 1..3 {
        party_members.push(H160::random());
    }

    RecoveryParty {
        party_members,
        session_id: "mock recovery party session id".to_string(),
        bls_encryption_key,
        ecdsa_encryption_key,
        threshold: 2,
    }
}

#[cfg(test)]
mod test {
    use crate::endpoints::admin::utils::{
        encrypt_and_tar_backup_keys, get_test_recovery_party, spawn_child_and_check,
        untar_keys_stream,
    };
    use crate::tests::key_shares::{TEST_BLS_KEY_SHARE, TEST_ECDSA_KEY_SHARE};
    use crate::tests::key_shares::{TEST_BLS_PUB_KEY, TEST_ECDSA_PUB_KEY};
    use crate::tss::common::backup::{RecoveryParty, VerifiableBackup};
    use crate::tss::common::curve_type::CurveType;
    use crate::tss::common::key_share::KeyShare;
    use crate::tss::common::key_share_helper::KeyHelper;
    use crate::tss::common::restore::RestoreState;
    use crate::tss::common::storage::write_backup_to_disk;
    use crate::tss::common::traits::key_persistence::KeyPersistence;
    use crate::utils::encoding::BeBytes;
    use blsful::inner_types::{Bls12381G1, G1Projective};
    use bulletproofs::BulletproofCurveArithmetic as BCA;
    use k256::{ProjectivePoint, PublicKey, Secp256k1};
    use tokio::process::{Child, Command};
    use verifiable_share_encryption::{DecryptionShare, VerifiableEncryptionDecryptor};
    type BlsShare = KeyShare;
    type EcdsaShare = KeyShare;

    // These tests need to run sequentially not to interfere with each other.
    #[test]
    fn run_backup_tests() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            test_encrypt_tar_and_untar_backup_keys().await;
        });
        rt.block_on(async {
            test_untar_old_backup().await;
        });
    }

    // Helper function
    pub fn get_test_recovery_party_with_encryption_keys() -> RecoveryParty {
        let mut recovery_party = get_test_recovery_party();
        recovery_party.bls_encryption_key = G1Projective::from_compressed(
            hex::decode(TEST_BLS_PUB_KEY)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        recovery_party.ecdsa_encryption_key = ProjectivePoint::from(
            PublicKey::from_sec1_bytes(&hex::decode(TEST_ECDSA_PUB_KEY).unwrap()).unwrap(),
        );
        recovery_party
    }

    async fn test_encrypt_tar_and_untar_backup_keys() {
        let cfg = crate::tests::common::get_backup_config();
        let pubkey = "test_encrypt_and_tar_backup_keys_pubkey";
        let staker_address = &crate::endpoints::recovery::get_staker_address(&cfg)
            .expect("Failed to get staker address");
        // Make sure that there are at least one ECDSA and one BLS key share.
        let bls_key: BlsShare = serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let ecdsa_key: EcdsaShare = serde_json::from_str(TEST_ECDSA_KEY_SHARE).unwrap();

        write_backup_to_disk(pubkey, 0, CurveType::BLS, &bls_key, &[], staker_address)
            .await
            .unwrap();
        write_backup_to_disk(pubkey, 0, CurveType::K256, &ecdsa_key, &[], staker_address)
            .await
            .unwrap();

        // Call the function to be tested
        let (bls_blinder, ecdsa_blinder) = RestoreState::generate_blinders();
        let recovery_party = get_test_recovery_party_with_encryption_keys();

        let child =
            encrypt_and_tar_backup_keys(&cfg, &bls_blinder, &ecdsa_blinder, &recovery_party, None)
                .await
                .unwrap();

        let mut restore_state = RestoreState::new(&cfg).unwrap();
        restore_state.actively_restoring = true;
        let restore_state = tokio::sync::RwLock::new(restore_state);
        untar_keys_stream(&cfg, &restore_state, child.stdout.unwrap())
            .await
            .unwrap();

        // Make sure the keys are also loaded.
        let state = restore_state.read().await;
        let encrypted_bls_key = state
            .fetch_bls_backup_by_pubkey_in_filename(pubkey)
            .expect("Encrypted BLS key share is not found");
        let encrypted_ecdsa_key = state
            .fetch_ecdsa_backup_by_pubkey_in_filename(pubkey)
            .expect("Encrypted ECDSA key share is not found");

        let bls_key_helper = KeyHelper::<G1Projective>::default();
        let k256_key_helper = KeyHelper::<ProjectivePoint>::default();

        let bls_key_private_share = decrypt_bls_share(encrypted_bls_key, &bls_blinder);
        assert_eq!(
            bls_key_private_share,
            bls_key_helper
                .secret_from_hex(&bls_key.hex_private_share)
                .unwrap()
        );

        let ecdsa_key_private_share = decrypt_ecdsa_share(encrypted_ecdsa_key, &ecdsa_blinder);

        assert_eq!(
            ecdsa_key_private_share,
            k256_key_helper
                .secret_from_hex(&ecdsa_key.hex_private_share)
                .unwrap()
        )
    }

    // Helper function
    fn decrypt_bls_share(
        vb: &VerifiableBackup<Bls12381G1>,
        blinder: &<Bls12381G1 as BCA>::Scalar,
    ) -> <Bls12381G1 as BCA>::Scalar {
        use crate::tests::key_shares::{
            hex_to_bls_dec_key_share, TEST_BLS_PRI_KEY_SHARE_1, TEST_BLS_PRI_KEY_SHARE_2,
        };
        let dec_key_share_1 = hex_to_bls_dec_key_share(TEST_BLS_PRI_KEY_SHARE_1, 1);
        let dec_key_share_2 = hex_to_bls_dec_key_share(TEST_BLS_PRI_KEY_SHARE_2, 2);

        let decryption_share_1 =
            DecryptionShare::<Vec<u8>, Bls12381G1>::new(&dec_key_share_1, &vb.ciphertext);
        let decryption_share_2 =
            DecryptionShare::<Vec<u8>, Bls12381G1>::new(&dec_key_share_2, &vb.ciphertext);

        Bls12381G1::decrypt_with_shares_and_unblind(
            blinder,
            &[decryption_share_1, decryption_share_2],
            &vb.ciphertext,
        )
        .unwrap()
    }

    // Helper function
    fn decrypt_ecdsa_share(
        vb: &VerifiableBackup<Secp256k1>,
        blinder: &<Secp256k1 as BCA>::Scalar,
    ) -> <Secp256k1 as BCA>::Scalar {
        use crate::tests::key_shares::{
            hex_to_ecdsa_dec_key_share, TEST_ECDSA_PRI_KEY_SHARE_1, TEST_ECDSA_PRI_KEY_SHARE_2,
        };
        let dec_key_share_1 = hex_to_ecdsa_dec_key_share(TEST_ECDSA_PRI_KEY_SHARE_1, 1);
        let dec_key_share_2 = hex_to_ecdsa_dec_key_share(TEST_ECDSA_PRI_KEY_SHARE_2, 2);

        let decryption_share_1 =
            DecryptionShare::<Vec<u8>, Secp256k1>::new(&dec_key_share_1, &vb.ciphertext);
        let decryption_share_2 =
            DecryptionShare::<Vec<u8>, Secp256k1>::new(&dec_key_share_2, &vb.ciphertext);

        Secp256k1::decrypt_with_shares_and_unblind(
            blinder,
            &[decryption_share_1, decryption_share_2],
            &vb.ciphertext,
        )
        .unwrap()
    }

    // Helper function
    async fn read_old_backup_tar_file() -> Child {
        use std::process::Stdio;
        let mut cmd = Command::new("cat");
        cmd.arg("tests/backup.tar");

        spawn_child_and_check(
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit()),
            "unable to tar the backup",
        )
        .await
        .unwrap()
    }

    async fn test_untar_old_backup() {
        use crate::tests::key_shares::{TEST_BLS_BLINDER, TEST_ECDSA_BLINDER};
        let bls_blinder = blsful::inner_types::Scalar::from_be_bytes(
            &(hex::decode(TEST_BLS_BLINDER).unwrap()).try_into().unwrap(),
        )
        .unwrap();
        let ecdsa_blinder =
            k256::Scalar::from_be_bytes(&hex::decode(TEST_ECDSA_BLINDER).unwrap()).unwrap();
        let pubkey = "test_untar_old_backup";

        let cfg = crate::tests::common::get_backup_config();
        let child = read_old_backup_tar_file();

        // Untar and load the old backup
        let recovery_party = get_test_recovery_party_with_encryption_keys();
        let mut restore_state = RestoreState::new(&cfg).unwrap();
        restore_state.actively_restoring = true;
        let restore_state = tokio::sync::RwLock::new(restore_state);
        untar_keys_stream(&cfg, &restore_state, child.await.stdout.unwrap())
            .await
            .unwrap();

        // Make sure the keys are loaded.
        let state = restore_state.read().await;
        let encrypted_bls_key = state
            .fetch_bls_backup_by_pubkey_in_filename(pubkey)
            .expect("Encrypted BLS key share is not found");
        let encrypted_ecdsa_key = state
            .fetch_ecdsa_backup_by_pubkey_in_filename(pubkey)
            .expect("Encrypted ECDSA key share is not found");

        // Check that the private shares are correctly decrypted.
        let bls_key: BlsShare = serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let ecdsa_key: EcdsaShare = serde_json::from_str(TEST_ECDSA_KEY_SHARE).unwrap();

        let bls_key_helper = KeyHelper::<G1Projective>::default();
        let k256_key_helper = KeyHelper::<ProjectivePoint>::default();

        let bls_key_private_share = decrypt_bls_share(encrypted_bls_key, &bls_blinder);
        assert_eq!(
            bls_key_private_share,
            bls_key_helper
                .secret_from_hex(&bls_key.hex_private_share)
                .unwrap()
        );

        let ecdsa_key_private_share = decrypt_ecdsa_share(encrypted_ecdsa_key, &ecdsa_blinder);
        assert_eq!(
            ecdsa_key_private_share,
            k256_key_helper
                .secret_from_hex(&ecdsa_key.hex_private_share)
                .unwrap()
        );
    }
}
