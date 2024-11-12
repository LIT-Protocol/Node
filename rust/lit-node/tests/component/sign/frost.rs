use std::marker::PhantomData;

use crate::component::dkg::initial_dkg;
use elliptic_curve::group::GroupEncoding;
use elliptic_curve::Group;
use futures::future::join_all;
use k256;
use lit_frost::{Identifier, SignatureShare, SigningCommitments, VerifyingKey, VerifyingShare};
use lit_node::peers::peer_state::models::SimplePeerExt;
use lit_node::peers::utils::derministic_subset::DeterministicSubset;
use lit_node::tss::common::dkg_type::DkgType;
use lit_node::tss::common::signing_scheme::SigningScheme;
use lit_node::tss::common::tss_state::TssState;
use lit_node::tss::frost::FrostState;
use test_case::test_case;
use test_common::interpolation::load_key_share;
use tokio::task::JoinHandle;
use tracing::info;
use vsss_rs::curve25519::{WrappedEdwards, WrappedRistretto};

#[test_case(SigningScheme::SchnorrEd25519Sha512; "Sign using Ed25519")]
#[test_case(SigningScheme::SchnorrK256Sha256;  "Sign using K256")]
#[test_case(SigningScheme::SchnorrP256Sha256;  "Sign using P256")]
#[test_case(SigningScheme::SchnorrP384Sha384;  "Sign using P384")]
#[test_case(SigningScheme::SchnorrRistretto25519Sha512;  "Sign using Ristretto")]
#[test_case(SigningScheme::SchnorrEd448Shake256;  "Sign using Ed448")]
// #[test_case(SigningScheme::SchnorrRedJubjubBlake2b512;  "Sign using RedJubJub")]
#[test_case(SigningScheme::SchnorrK256Taproot;  "Sign using Taproot")]
#[tokio::test]
#[doc = "Test Frost signatures using a set of virtual nodes."]
pub async fn sign_with_pubkey(signing_scheme: SigningScheme) {
    match signing_scheme {
        SigningScheme::SchnorrEd25519Sha512 => {
            sign_with_typeof_pubkey::<WrappedEdwards>(
                signing_scheme,
                lit_frost::Scheme::Ed25519Sha512,
            )
            .await
        }
        SigningScheme::SchnorrK256Sha256 => {
            sign_with_typeof_pubkey::<k256::ProjectivePoint>(
                signing_scheme,
                lit_frost::Scheme::K256Sha256,
            )
            .await
        }
        SigningScheme::SchnorrP256Sha256 => {
            sign_with_typeof_pubkey::<p256::ProjectivePoint>(
                signing_scheme,
                lit_frost::Scheme::P256Sha256,
            )
            .await
        }
        SigningScheme::SchnorrP384Sha384 => {
            sign_with_typeof_pubkey::<p384::ProjectivePoint>(
                signing_scheme,
                lit_frost::Scheme::P384Sha384,
            )
            .await
        }
        SigningScheme::SchnorrRistretto25519Sha512 => {
            sign_with_typeof_pubkey::<WrappedRistretto>(
                signing_scheme,
                lit_frost::Scheme::Ristretto25519Sha512,
            )
            .await
        }
        SigningScheme::SchnorrEd448Shake256 => {
            sign_with_typeof_pubkey::<ed448_goldilocks::EdwardsPoint>(
                signing_scheme,
                lit_frost::Scheme::Ed448Shake256,
            )
            .await
        }
        SigningScheme::SchnorrRedJubjubBlake2b512 => {
            sign_with_typeof_pubkey::<jubjub::SubgroupPoint>(
                signing_scheme,
                lit_frost::Scheme::RedJubjubBlake2b512,
            )
            .await
        }
        SigningScheme::SchnorrK256Taproot => {
            sign_with_typeof_pubkey::<k256::ProjectivePoint>(
                signing_scheme,
                lit_frost::Scheme::K256Taproot,
            )
            .await
        }
        _ => panic!("Unsupported curve type"),
    }
}

pub async fn sign_with_typeof_pubkey<G>(
    signing_scheme: SigningScheme,
    aggregation_scheme: lit_frost::Scheme,
) where
    G: Group + GroupEncoding + Default,
{
    test_common::init_test_config();
    info!("Starting test: sign with frost using {:?}", &signing_scheme);
    let num_nodes = 5;
    let curve_type = signing_scheme.curve_type();

    let (vnc, pubkey, epoch, peers) = initial_dkg(curve_type, num_nodes).await;
    let threshold = peers.threshold_for_set();
    let request_id = "1234".as_bytes();

    let txn_prefix = "FROST_SIGN";
    let message = "Hello world!".as_bytes();
    let mut signing_peers = DeterministicSubset::new_from_peer_sets(peers.clone())
        .get_subset(&message, request_id, threshold as usize)
        .unwrap();
    signing_peers.set_all_protocol_indices(1);

    //TEST  let signing_peers be the first two peers
    info!(
        "peers: \n  all:{:?}  \nsubset:{:?}",
        peers.debug_addresses(),
        signing_peers.debug_addresses()
    );

    let mut v = Vec::new();
    let all_signing_peers = signing_peers.clone();
    for signing_node in signing_peers {
        let index = signing_node.share_index as usize;
        let frost_state = get_frost_state::<G>(vnc.nodes[index].tss_state.clone());

        let (_, secret_share, verifying_key, _) =
            load_key_share(&signing_node, &pubkey, epoch, curve_type).await;

        let peers = all_signing_peers.clone();

        let signing_scheme = signing_scheme.clone();

        let jh: JoinHandle<(
            Identifier,
            SignatureShare,
            SigningCommitments,
            VerifyingShare,
            VerifyingKey,
        )> = tokio::task::spawn(async move {
            let sig_share = frost_state
                .sign_internal(
                    txn_prefix,
                    peers,
                    message,
                    signing_scheme,
                    verifying_key,
                    secret_share,
                    threshold as u16,
                )
                .await;
            sig_share.expect("error from frost state sig_share")
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;

    let mut signing_commitments: Vec<(Identifier, SigningCommitments)> = Vec::new();
    let mut signature_shares: Vec<(Identifier, SignatureShare)> = Vec::new();
    let mut signer_pubkeys: Vec<(Identifier, VerifyingShare)> = Vec::new();
    let mut verifying_key: VerifyingKey = VerifyingKey::default();
    for result in &results {
        assert!(result.is_ok());
        let (identifier, signature_share, commitments, verifying_share, verifying_key1) =
            result.as_ref().expect("error unwrapping result");
        signing_commitments.push((identifier.clone(), commitments.clone()));
        signature_shares.push((identifier.clone(), signature_share.clone()));
        signer_pubkeys.push((identifier.clone(), verifying_share.clone()));

        if verifying_key == VerifyingKey::default() {
            verifying_key = verifying_key1.clone();
        }
    }

    tracing::info!("Signature shares: {:?}", signature_shares);
    tracing::info!("Signing commitments: {:?}", signing_commitments);
    tracing::info!("Signer pubkeys: {:?}", signer_pubkeys);

    let signature = aggregation_scheme
        .aggregate(
            message,
            &signing_commitments,
            &signature_shares,
            &signer_pubkeys,
            &verifying_key,
        )
        .expect("error aggregating signature");

    tracing::info!("Signature: {:?}", signature);

    let r = aggregation_scheme.verify(message, &verifying_key, &signature);

    assert!(r.is_ok());

    tracing::info!("Verification: {:?}", r);
}

fn get_frost_state<G: Group + GroupEncoding + Default>(state: TssState) -> FrostState<G> {
    FrostState {
        state,
        dkg_type: DkgType::Standard,
        _phantom: PhantomData,
    }
}
