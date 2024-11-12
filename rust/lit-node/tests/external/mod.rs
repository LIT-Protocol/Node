use std::str::FromStr;

use super::integration::ecdsa::new_node_collection_with_authd_pkp_with_token;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use lit_core::utils::binary::bytes_to_hex;
use maplit::hashset;
use rocket::State;
use test_common::pkp::add_permitted_address_to_pkp;

use rocket::fairing::AdHoc;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions, Method};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct SecretState {
    wallet_address: String,
    wallet_secret: String,
    pubkey: String,
}

#[tokio::test]
#[ignore]
#[doc = "This test is used to load a network for external tests.  It will run for a little more than 1 day, and is not intended to be run as part of the test suite."]
async fn load_network_for_external_tests() {
    test_common::init_test_config();
    let num_nodes = 3;
    let (_testnet, validator_collection, pubkey, token_id) =
        new_node_collection_with_authd_pkp_with_token(num_nodes, false).await;

    let wallet = LocalWallet::new(&mut rand_core::OsRng);
    let address = &bytes_to_hex(wallet.address().to_fixed_bytes());

    let _res = add_permitted_address_to_pkp(
        validator_collection.actions(),
        address,
        token_id,
        &[U256::from(1)],
    )
    .await;

    info!("Started network for external tests");

    let secret = bytes_to_hex(wallet.signer().as_nonzero_scalar().to_bytes());
    info!("Wallet address that controls a minted PKP: {}", address);
    info!("Secret that controls a minted PKP: {}", secret);

    let state = SecretState {
        wallet_address: address.clone(),
        wallet_secret: secret.clone(),
        pubkey: pubkey.clone(),
    };

    let allowed_methods = hashset! {
    Method::from_str("Get").unwrap(),
    Method::from_str("Post").unwrap(),
    Method::from_str("Patch").unwrap()};

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed to build");

    let r = rocket::build()
        .manage(state)
        .manage(cors)
        .mount("/", routes![index]);
    r.launch().await.unwrap();
    // tokio::time::sleep(std::time::Duration::from_secs(100000)).await;
}

#[get("/")]
fn index(secret_state: &State<SecretState>) -> serde_json::Value {
    serde_json::json!(secret_state.inner())
}
