use blsful::inner_types::{Bls12381G1, ExpandMsgXmd, Field, G1Projective, Scalar as BlsScalar};
use ethers::types::Address;
use gennaro_dkg::vsss_rs::{self, elliptic_curve::hash2curve::GroupDigest};
use k256::{
    ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey},
    FieldBytes, ProjectivePoint, Scalar as K256Scalar, Secp256k1,
};
use sha3::{digest::Digest, Keccak256};
use verifiable_share_encryption::{
    Ciphertext, DecryptionShare, Proof, VerifiableEncryption, VerifiableEncryptionDecryptor,
};
use vsss_rs::Share;

use lit_node::utils::consensus::get_threshold_count;

use test_common::new_node_collection;

#[tokio::test]
async fn end_to_end_backup_test() {
    const NODE_COUNT: usize = 3;
    const ROOT_KEY_COUNT: usize = 10;

    // step 0: init and load the test configuration file
    test_common::init_test_config();
    let (_testnet, validator_collection) = new_node_collection(NODE_COUNT, false).await;

    // Step 1: Get registered wallet addresses and
    // node operators create blinders
    let recovery_party_addresses = mock_get_wallet_addresses(None);
    let blinders = mock_get_blinders(NODE_COUNT);

    let backup_recovery_contract = &validator_collection.actions().contracts().backup_recovery;
    backup_recovery_contract
        .register_new_backup_party(recovery_party_addresses.clone())
        .call()
        .await
        .unwrap();

    // Step 2: Nodes create encryption shares for each wallet address
    let (
        (bls_encryption_key, bls_decryption_shares),
        (k256_encryption_key, _k256_decryption_shares),
    ) = mock_recovery_dkg(recovery_party_addresses.len());

    // Step 3: Wallets download their shares and submit proofs to the smart contract
    if mock_can_delete_decryption_shares() {
        // TODO: delete decryption shares
    }

    // Step 4: Nodes create their signing shares
    let ((_bls_pub_key, bls_key_shares), (_root_pub_keys, root_signing_shares)) =
        mock_get_signing_shares(NODE_COUNT);

    // Step 5: Create verifiable backups and upload them to a public storage
    let mut bls_verifiable_backups = Vec::with_capacity(NODE_COUNT);
    let mut k256_verifiable_backups = Vec::with_capacity(NODE_COUNT);

    // NOTE: Normally this is what we'd do, however its slow and this is a test
    // If one succeeds they all should so we just use the first one
    // for i in 0..NODE_COUNT {
    //     let bls_backup = mock_create_bls_verifiable_backup(
    //         bls_encryption_key,
    //         &bls_key_shares[i],
    //         &blinders[i].0,
    //     );
    //     bls_verifiable_backups.push(bls_backup);
    //     let mut root_key_backups = Vec::with_capacity(ROOT_KEY_COUNT);
    //     for j in 0..ROOT_KEY_COUNT {
    //         let root_key_backup = mock_create_k256_verifiable_backup(
    //             k256_encryption_key,
    //             &root_signing_shares[i][j],
    //             &blinders[i].1,
    //         );
    //         root_key_backups.push(root_key_backup);
    //     }
    //     k256_verifiable_backups.push(root_key_backups);
    // }
    let bls_backup =
        mock_create_bls_verifiable_backup(bls_encryption_key, &bls_key_shares[0], &blinders[0].0);
    bls_verifiable_backups.push(bls_backup);
    let mut root_key_backups = Vec::with_capacity(ROOT_KEY_COUNT);
    let root_key_backup = mock_create_k256_verifiable_backup(
        k256_encryption_key,
        &root_signing_shares[0][0],
        &blinders[0].1,
    );
    root_key_backups.push(root_key_backup);
    k256_verifiable_backups.push(root_key_backups);

    // Verify the backup proofs
    bls_verifiable_backups
        .iter()
        .enumerate()
        .for_each(|(i, (ciphertext, proof))| {
            let key_share =
                <Vec<u8> as Share>::as_field_element::<BlsScalar>(&bls_key_shares[i]).unwrap();
            let verification_key = G1Projective::GENERATOR * (key_share + &blinders[i].0);
            assert!(Bls12381G1::verify(
                bls_encryption_key,
                verification_key,
                ciphertext,
                proof,
                b"mock_create_bls_verifiable_backup",
            )
            .is_ok());
        });

    k256_verifiable_backups
        .iter()
        .enumerate()
        .for_each(|(i, shares)| {
            shares
                .iter()
                .enumerate()
                .for_each(|(j, (ciphertext, proof))| {
                    let key_share = <Vec<u8> as Share>::as_field_element::<K256Scalar>(
                        &root_signing_shares[i][j],
                    )
                    .unwrap();
                    let verification_key = ProjectivePoint::GENERATOR * (key_share + blinders[i].1);
                    assert!(Secp256k1::verify(
                        k256_encryption_key,
                        verification_key,
                        ciphertext,
                        proof,
                        b"mock_create_k256_verifiable_backup",
                    )
                    .is_ok());
                });
        });

    // Some time later
    // Step 6: Node is in recovery state and needs the backups
    // The blinder is uploaded and the RP wallets are queried for decryption shares
    let mut bls_0_decryption_shares = Vec::with_capacity(bls_decryption_shares.len());

    for share in bls_decryption_shares.iter() {
        let decryption_share: DecryptionShare<Vec<u8>, Bls12381G1> =
            DecryptionShare::new(share, &bls_verifiable_backups[0].0);
        bls_0_decryption_shares.push(decryption_share);
    }
    let res = Bls12381G1::decrypt_with_shares_and_unblind(
        &blinders[0].0,
        bls_0_decryption_shares.as_slice(),
        &bls_verifiable_backups[0].0,
    );
    assert!(res.is_ok());
    let recovered_share0 = res.unwrap();
    assert_eq!(
        recovered_share0,
        <Vec<u8> as Share>::as_field_element(&bls_key_shares[0]).unwrap()
    );
}

fn mock_create_bls_verifiable_backup(
    encryption_key: G1Projective,
    signing_share: &Vec<u8>,
    blinder: &BlsScalar,
) -> (Ciphertext<Bls12381G1>, Proof<Bls12381G1>) {
    let rng = mock_rng(Some(&[43u8; 32]));
    let key_share = <Vec<u8> as Share>::as_field_element::<BlsScalar>(signing_share).unwrap();
    Bls12381G1::blind_encrypt_and_prove(
        encryption_key,
        &key_share,
        blinder,
        b"mock_create_bls_verifiable_backup",
        rng,
    )
}

fn mock_create_k256_verifiable_backup(
    encryption_key: ProjectivePoint,
    decryption_share: &Vec<u8>,
    blinder: &K256Scalar,
) -> (Ciphertext<Secp256k1>, Proof<Secp256k1>) {
    let rng = mock_rng(Some(&[47u8; 32]));
    let key_share = <Vec<u8> as Share>::as_field_element::<K256Scalar>(decryption_share).unwrap();
    Secp256k1::blind_encrypt_and_prove(
        encryption_key,
        &key_share,
        blinder,
        b"mock_create_k256_verifiable_backup",
        rng,
    )
}

fn mock_get_signing_shares(
    node_count: usize,
) -> (
    (G1Projective, Vec<Vec<u8>>),
    (Vec<ProjectivePoint>, Vec<Vec<Vec<u8>>>),
) {
    // TODO: replace with real method when available
    let threshold = get_threshold_count(node_count);
    const SECRET_MSG: &[u8] = b"secret signing message";
    let mut rng = mock_rng(Some(&[31u8; 32]));
    let bls_keys = create_bls_keys_and_shares(SECRET_MSG, threshold, node_count, &mut rng);
    let mut root_pub_keys = Vec::with_capacity(10);
    let mut root_secret_shares = Vec::with_capacity(10);

    for _ in 0..10 {
        let (root_pub_key, root_secret_share) =
            create_k256_keys_and_shares(SECRET_MSG, threshold, node_count, &mut rng);
        root_pub_keys.push(root_pub_key);
        root_secret_shares.push(root_secret_share);
    }
    (bls_keys, (root_pub_keys, root_secret_shares))
}

fn mock_can_delete_decryption_shares() -> bool {
    // TODO: replace with real method when available
    true
}

fn mock_recovery_dkg(
    recovery_party_count: usize,
) -> (
    (G1Projective, Vec<Vec<u8>>),
    (ProjectivePoint, Vec<Vec<u8>>),
) {
    // TODO: replace with real method when available
    let threshold = get_threshold_count(recovery_party_count);
    const SECRET_MSG: &[u8] = b"secret encrypting message";

    let mut rng = mock_rng(Some(&[29u8; 32]));

    (
        create_bls_keys_and_shares(SECRET_MSG, threshold, recovery_party_count, &mut rng),
        create_k256_keys_and_shares(SECRET_MSG, threshold, recovery_party_count, &mut rng),
    )
}

fn mock_get_wallet_addresses(verifying_keys: Option<Vec<VerifyingKey>>) -> Vec<Address> {
    // TODO: replace with real method when available
    let verifying_keys = verifying_keys.unwrap_or_else(|| mock_get_verifying_keys(None));
    verifying_keys
        .iter()
        .map(|key| {
            key.to_eth_address()
                .parse()
                .expect("Could not convert address string to bytes")
        })
        .collect()
}

fn mock_get_verifying_keys(signing_keys: Option<Vec<SigningKey>>) -> Vec<VerifyingKey> {
    signing_keys
        .unwrap_or_else(mock_get_signing_keys)
        .iter()
        .map(|key| key.verifying_key())
        .copied()
        .collect()
}

fn mock_get_signing_keys() -> Vec<SigningKey> {
    // TODO: replace with real method when available
    let signing_key_bytes = [[3u8; 32], [5u8; 32], [7u8; 32], [11u8; 32], [13u8; 32]];
    signing_key_bytes
        .iter()
        .map(|bytes| {
            let bytes = FieldBytes::from_slice(bytes);
            SigningKey::from_bytes(bytes).unwrap()
        })
        .collect()
}

fn mock_get_blinders(node_count: usize) -> Vec<(BlsScalar, K256Scalar)> {
    let mut rng = mock_rng(Some(&[41u8; 32]));
    (0..node_count)
        .map(|_| (BlsScalar::random(&mut rng), K256Scalar::random(&mut rng)))
        .collect()
}

fn mock_rng(seed: Option<&[u8; 32]>) -> rand_chacha::ChaChaRng {
    use rand::SeedableRng;

    let seed = seed.unwrap_or(&[23u8; 32]);
    rand_chacha::ChaChaRng::from_seed(*seed)
}

fn create_bls_keys_and_shares(
    seed_msg: &[u8],
    threshold: usize,
    limit: usize,
    rng: &mut rand_chacha::ChaChaRng,
) -> (G1Projective, Vec<Vec<u8>>) {
    let bls_secret =
        BlsScalar::hash::<ExpandMsgXmd<sha2::Sha256>>(seed_msg, b"BLS12381_XMD:SHA-256_SSWU_RO_");
    let bls_public = G1Projective::GENERATOR * bls_secret;
    let bls_secret_shares =
        vsss_rs::shamir::split_secret::<BlsScalar, u8, Vec<u8>>(threshold, limit, bls_secret, rng)
            .unwrap();
    (bls_public, bls_secret_shares)
}

fn create_k256_keys_and_shares(
    seed_msg: &[u8],
    threshold: usize,
    limit: usize,
    rng: &mut rand_chacha::ChaChaRng,
) -> (ProjectivePoint, Vec<Vec<u8>>) {
    let k256_secret = Secp256k1::hash_to_scalar::<ExpandMsgXmd<sha2::Sha256>>(
        &[seed_msg],
        &[b"secp256k1_XMD:SHA-256_SSWU_RO_"],
    )
    .unwrap();
    let k256_public = ProjectivePoint::GENERATOR * k256_secret;
    let k256_secret_shares = vsss_rs::shamir::split_secret::<K256Scalar, u8, Vec<u8>>(
        threshold,
        limit,
        k256_secret,
        rng,
    )
    .unwrap();
    (k256_public, k256_secret_shares)
}

trait EthereumAddress {
    fn to_eth_address(&self) -> String;
}

trait EthereumSignature {
    fn sign_eth(&self, pre_hash: &[u8]) -> (Signature, RecoveryId);
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> String {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).unwrap();
        let address = fmt_address(&last_20);
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(&String::from_utf8(address.to_vec()).unwrap());
        buffer
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> String {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

impl EthereumSignature for SigningKey {
    fn sign_eth(&self, message: &[u8]) -> (Signature, RecoveryId) {
        let digest = keccak256(message);
        self.sign_prehash_recoverable(&digest).unwrap()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> [u8; 40] {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer).unwrap();

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    buffer
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}
