use blsful::{vsss_rs::Share, Bls12381G2Impl, Pairing, SecretKeyShare};
use bulletproofs::bls12_381_plus::G1Projective;

use lit_node::config::segmented_paths;
use lit_node::error::Result;
use lit_node::tss::common::key_type::KeyType;
use lit_node::tss::common::traits::keyresolver::KeyResolver;
use lit_node::tss::ecdsa_cait_sith::models::EcdsaKeyShare;
use lit_node::tss::{blsful::models::BlsKeyShare, common::storage::do_read_from_disk};
use vsss_rs::combine_shares;

#[derive(Debug, Clone, PartialEq)]
pub enum SecretScalarType {
    BLS(bulletproofs::bls12_381_plus::Scalar),
    Ecdsa(k256::Scalar),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecretKeyShareType {
    BLS(BlsKeyShare<G1Projective>),
    Ecdsa(EcdsaKeyShare<k256::Secp256k1>),
}

pub async fn interpolate_secret(
    key_type: KeyType,
    staker_addresses: &Vec<(String, bool)>,
    threshold: usize,
    pubkey: &str,
    epoch: u64,
) -> SecretScalarType {
    match key_type {
        KeyType::BLS => {
            let secret = interpolate_bls_secret(staker_addresses, threshold, pubkey, epoch).await;
            SecretScalarType::BLS(secret)
        }
        KeyType::EcdsaCaitSith => {
            let secret = interpolate_ecdsa_secret(staker_addresses, threshold, pubkey, epoch).await;
            SecretScalarType::Ecdsa(secret)
        }
    }
}

pub async fn load_secret_shares(
    key_type: KeyType,
    pubkey: &str,
    staker_addresses: &Vec<(String, bool)>,
    epoch: u64,
) -> Vec<SecretKeyShareType> {
    let staker_addresses = &staker_addresses
        .iter()
        .map(|(address, _)| address.to_string())
        .collect::<Vec<String>>();
    match key_type {
        KeyType::BLS => {
            let shares = load_bls_shares(staker_addresses, pubkey, epoch).await;

            shares
                .into_iter()
                .map(SecretKeyShareType::BLS)
                .collect::<Vec<SecretKeyShareType>>()
        }
        KeyType::EcdsaCaitSith => {
            let shares = load_ecdsa_key_shares(pubkey, staker_addresses, epoch).await;

            shares
                .into_iter()
                .map(SecretKeyShareType::Ecdsa)
                .collect::<Vec<SecretKeyShareType>>()
        }
    }
}

// this function duplicates how a node determines key folders.   Because the test doesn't run in the context
// of a node (unless it's a compoenent test), we need to use the staker address as a param to find the folder.
pub async fn read_key_share_from_disk_from_test_harness<T>(
    staker_addresses: &Vec<String>,
    pubkey: &str,
    share_index: u16,
    epoch: u64,
    key_type: KeyType,
) -> Result<T>
where
    T: serde::de::DeserializeOwned + KeyResolver,
{
    let prefix = "Key";
    let key_type_folder = match key_type {
        KeyType::BLS => "bls",
        KeyType::EcdsaCaitSith => "ecdsa",
    };
    let key_type = key_type as u8;
    let key_name = format!(
        "{}-H-{}-{}-{}-H-{}.cbor",
        prefix, key_type, pubkey, share_index, epoch
    );

    let mut staker_addresses = staker_addresses.clone();
    staker_addresses.sort();
    let staker_address = staker_addresses
        .iter()
        .nth(share_index as usize)
        .unwrap()
        .to_lowercase();

    let root_dir = match staker_address.is_empty() {
        true => format!("./node_keys/{}", key_type_folder),
        false => format!("./node_keys/0x{}/{}", staker_address, key_type_folder),
    };

    let mut path = segmented_paths(root_dir, pubkey, 3, true)?;
    path.push(key_name);
    do_read_from_disk(&path).await
}

pub async fn load_bls_secret_share(
    staker_addresses: &Vec<String>,
    pubkey: &str,
    share_index: u16,
    epoch: u64,
) -> SecretKeyShare<Bls12381G2Impl> {
    let bls_key_share = read_key_share_from_disk_from_test_harness::<BlsKeyShare<G1Projective>>(
        staker_addresses,
        pubkey,
        share_index,
        epoch,
        KeyType::BLS,
    )
    .await;

    let bls_key_share = match bls_key_share {
        Ok(bls_key_share) => bls_key_share,
        Err(e) => {
            panic!("Error loading keyshare: {:?}", e);
        }
    };

    println!("Share: {:?}", &bls_key_share);

    let identifier = bls_key_share.index as u8;
    let identifier = identifier + 1;
    let value = bls_key_share.private_share.to_le_bytes();

    SecretKeyShare(
        <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(identifier, &value),
    )
}

pub async fn load_bls_shares(
    staker_addresses: &Vec<String>,
    pubkey: &str,
    epoch: u64,
) -> Vec<BlsKeyShare<G1Projective>> {
    let mut shares = Vec::new();
    let node_num = staker_addresses.len();
    for i in 0..node_num as u16 {
        let bls_key_share =
            read_key_share_from_disk_from_test_harness::<BlsKeyShare<G1Projective>>(
                staker_addresses,
                pubkey,
                i,
                epoch,
                KeyType::BLS,
            )
            .await
            .expect("Error loading keyshare");
        shares.push(bls_key_share);
    }

    shares
}

pub async fn interpolate_bls_secret(
    all_staker_addresses: &Vec<(String, bool)>,
    threshold: usize,
    pubkey: &str,
    epoch: u64,
) -> bulletproofs::bls12_381_plus::Scalar {
    let mut shares = Vec::new();
    let node_num = all_staker_addresses.len();

    let staker_addresses = &all_staker_addresses
        .iter()
        .map(|(address, _)| address.to_string())
        .collect::<Vec<String>>();

    for i in 0..node_num {
        if all_staker_addresses[i].1 {
            let share = load_bls_secret_share(staker_addresses, pubkey, i as u16, epoch).await;
            shares.push(share);
        }
    }

    interpolate_bls_secret_from_shares(threshold, shares)
}

fn interpolate_bls_secret_from_shares(
    threshold: usize,
    shares: Vec<SecretKeyShare<Bls12381G2Impl>>,
) -> bulletproofs::bls12_381_plus::Scalar {
    let r4shares: Vec<Vec<u8>> = shares
        .into_iter()
        .skip(0)
        .take(threshold)
        .map(|p| <Vec<u8> as Share>::with_identifier_and_value(p.0.identifier(), p.0.value()))
        .collect();
    let secret = combine_shares::<bulletproofs::bls12_381_plus::Scalar, u8, Vec<u8>>(&r4shares);
    println!("Interpolated secret is ok: {:?}", secret.is_ok());
    secret.unwrap()
}

pub async fn load_ecdsa_secret_share(
    staker_addresses: &Vec<String>,
    pubkey: &str,
    share_index: u16,
    epoch: u64,
) -> (u8, k256::Scalar) {
    let key_share = read_key_share_from_disk_from_test_harness::<EcdsaKeyShare<k256::Secp256k1>>(
        staker_addresses,
        pubkey,
        share_index,
        epoch,
        KeyType::EcdsaCaitSith,
    )
    .await;

    let key_share = key_share.unwrap();

    let identifier = key_share.index as u8;
    let identifier = identifier + 1;
    let value = key_share.private_share;

    (identifier, value)
}
pub async fn interpolate_ecdsa_secret(
    all_staker_addresses: &Vec<(String, bool)>,
    threshold: usize,
    pubkey: &str,
    epoch: u64,
) -> k256::Scalar {
    let staker_addresses = &all_staker_addresses
        .iter()
        .map(|(address, _)| address.to_string())
        .collect::<Vec<String>>();

    let node_num = all_staker_addresses.len();
    let mut shares = Vec::new();
    for i in 0..node_num {
        if !all_staker_addresses[i].1 {
            continue;
        }
        let (identifier, private_share) =
            load_ecdsa_secret_share(staker_addresses, pubkey, i as u16, epoch).await;
        let private_share = private_share.to_bytes().to_vec();
        let share = (identifier, private_share);
        shares.push(share);
    }

    interpolate_ecdsa_secret_from_shares(threshold, shares)
}

pub fn interpolate_ecdsa_secret_from_shares(
    threshold: usize,
    shares: Vec<(u8, Vec<u8>)>,
) -> k256::Scalar {
    let r4shares: Vec<Vec<u8>> = shares
        .into_iter()
        .skip(0)
        .take(threshold)
        .map(|p| <Vec<u8> as Share>::with_identifier_and_value(p.0, &p.1))
        .collect();

    let secret = combine_shares::<k256::Scalar, u8, Vec<u8>>(&r4shares);

    assert!(secret.is_ok());

    println!("Interpolated secret is ok: {:?}", secret.is_ok());
    secret.unwrap()
}

pub async fn load_ecdsa_key_shares(
    pubkey: &str,
    staker_addresses: &Vec<String>,
    epoch: u64,
) -> Vec<EcdsaKeyShare<k256::Secp256k1>> {
    let mut shares = Vec::new();
    let node_num = staker_addresses.len();
    for i in 0..node_num as u16 {
        let key_share =
            read_key_share_from_disk_from_test_harness::<EcdsaKeyShare<k256::Secp256k1>>(
                staker_addresses,
                pubkey,
                i,
                epoch,
                KeyType::EcdsaCaitSith,
            )
            .await
            .expect("Err loading keyshare");

        shares.push(key_share);
    }

    shares
}
