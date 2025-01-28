use elliptic_curve::group::GroupEncoding;
use elliptic_curve::Group;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::config::segmented_paths;
use lit_node::error::Result;
use lit_node::peers::peer_state::models::SimplePeer;
use lit_node::tss::common::curve_type::CurveType;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::storage::do_read_from_disk;
use lit_node::utils::encoding::BeBytes;
use vsss_rs::combine_shares;
use vsss_rs::Share;

pub async fn get_secret_and_shares(
    curve_type: CurveType,
    pubkey: &str,
    peers: &Vec<SimplePeer>,
    epoch: u64,
) -> (Vec<u8>, Vec<Vec<u8>>) {
    let secret = interpolate_secret(curve_type, peers, pubkey, epoch).await;
    let shares = load_secret_shares(curve_type, pubkey, peers, epoch).await;
    (secret, shares)
}

pub async fn interpolate_secret(
    curve_type: CurveType,
    peers: &Vec<SimplePeer>,
    pubkey: &str,
    epoch: u64,
) -> Vec<u8> {
    let secret =
        match curve_type {
            CurveType::BLS => interpolate_secret_for_key::<blsful::inner_types::G1Projective>(
                peers, pubkey, epoch, curve_type,
            )
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::K256 => interpolate_secret_for_key::<k256::ProjectivePoint>(
                peers, pubkey, epoch, curve_type,
            )
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::P256 => interpolate_secret_for_key::<p256::ProjectivePoint>(
                peers, pubkey, epoch, curve_type,
            )
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::Ed25519 => interpolate_secret_for_key::<
                curve25519_dalek::edwards::SubgroupPoint,
            >(peers, pubkey, epoch, curve_type)
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::Ed448 => interpolate_secret_for_key::<ed448_goldilocks::EdwardsPoint>(
                peers, pubkey, epoch, curve_type,
            )
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::P384 => interpolate_secret_for_key::<p384::ProjectivePoint>(
                peers, pubkey, epoch, curve_type,
            )
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::Ristretto25519 => interpolate_secret_for_key::<
                curve25519_dalek::RistrettoPoint,
            >(peers, pubkey, epoch, curve_type)
            .await
            .to_be_bytes()
            .to_vec(),
            CurveType::RedJubjub => {
                let fr = interpolate_secret_for_key::<jubjub::SubgroupPoint>(
                    peers, pubkey, epoch, curve_type,
                )
                .await;
                let mut bytes = fr.to_bytes();
                bytes.reverse();
                bytes.to_vec()
            }
        };

    secret
}

pub async fn load_secret_shares(
    curve_type: CurveType,
    pubkey: &str,
    peers: &Vec<SimplePeer>,
    epoch: u64,
) -> Vec<Vec<u8>> {
    let mut shares: Vec<Vec<u8>> = Vec::new();

    for peer in peers {
        let (_, share, _, _) = load_key_share(peer, pubkey, epoch, curve_type).await;
        shares.push(share);
    }

    shares
}

// this function duplicates how a node determines key folders.   Because the test doesn't run in the context
// of a node (unless it's a compoenent test), we need to use the staker address as a param to find the folder.
pub async fn read_key_share_from_disk_from_test_harness<T>(
    peer: &SimplePeer,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let prefix = "Key";
    let share_index = peer.share_index;
    let curve_type_folder = curve_type.as_str();
    let curve_type = curve_type as u8;
    let key_name = format!(
        "{}-H-{}-{}-{}-H-{}.cbor",
        prefix, curve_type, pubkey, share_index, epoch
    );

    let staker_address = bytes_to_hex(peer.staker_address.as_bytes());

    let root_dir = match staker_address.is_empty() {
        true => format!("./node_keys/{}", curve_type_folder),
        false => format!("./node_keys/0x{}/{}", staker_address, curve_type_folder),
    };

    let mut path = segmented_paths(root_dir, pubkey, 3, true)?;
    path.push(key_name);
    do_read_from_disk(&path).await
}

pub async fn interpolate_secret_for_key<G>(
    peers: &Vec<SimplePeer>,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
) -> <G as Group>::Scalar
where
    G: Group + GroupEncoding + Default,
{
    let mut shares = Vec::new();
    let mut threshold = 0;
    for peer in peers {
        let (identifier, private_share, _public_key, share_threshold) =
            load_key_share(peer, pubkey, epoch, curve_type).await;
        if threshold == 0 {
            threshold = share_threshold as usize;
        }
        let share = (identifier, private_share);
        shares.push(share);
    }

    interpolate_secret_from_shares::<G>(threshold, shares)
}

pub async fn load_key_share(
    peer: &SimplePeer,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
) -> (u8, Vec<u8>, Vec<u8>, u16) {
    let key_share =
        read_key_share_from_disk_from_test_harness::<KeyShare>(peer, pubkey, epoch, curve_type)
            .await
            .expect("Failed to load key share");

    let private_share = key_share.secret_as_bytes(curve_type).unwrap();
    let public_key = key_share.public_key_as_bytes(curve_type).unwrap();
    let identifier = (key_share.index as u8) + 1;
    (identifier, private_share, public_key, key_share.threshold)
}

pub fn interpolate_secret_from_shares<G>(
    threshold: usize,
    shares: Vec<(u8, Vec<u8>)>,
) -> <G as Group>::Scalar
where
    G: Group + GroupEncoding + Default,
{
    tracing::info!("Interpolating secret from shares: {:?} ", shares);
    let r4shares: Vec<Vec<u8>> = shares
        .into_iter()
        // .skip(0)
        .take(threshold)
        .map(|p| <Vec<u8> as Share>::with_identifier_and_value(p.0, &p.1))
        .collect();

    let secret = combine_shares::<G::Scalar, u8, Vec<u8>>(&r4shares);

    assert!(secret.is_ok());

    secret.unwrap()
}
