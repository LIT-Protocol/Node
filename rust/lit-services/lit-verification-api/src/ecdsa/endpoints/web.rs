use lit_api_core::context::{with_context, Tracing};
use lit_core::utils::binary::bytes_to_hex;
use rocket::{get, http::Status, response::status, State};
use rocket_governor::RocketGovernor;
use serde_json::{json, Value};

use crate::{
    api_key::ApiKey,
    client_ip::ClientRealAddr,
    ecdsa::{models::PublicKeyResponse, SigningPair},
    limit::RateLimitGuard,
};

#[get("/api/otp/certs/checksum", format = "json")]
pub(crate) async fn get_key_eth(
    tracing: Tracing, _limitguard: RocketGovernor<'_, RateLimitGuard>,
    key_shares: &State<SigningPair>, _client_ip: ClientRealAddr, _api_key: ApiKey,
) -> status::Custom<Value> {
    with_context(tracing, async move {
        let addr = format!("0x{}", key_shares.public_key_checksum);
        status::Custom(Status::Ok, json!(PublicKeyResponse { public_key: addr }))
    })
    .await
}

#[get("/api/otp/certs/secp", format = "json")]
pub(crate) async fn get_key_secp(
    tracing: Tracing, _limitguard: RocketGovernor<'_, RateLimitGuard>,
    key_shares: &State<SigningPair>, _api_key: ApiKey, _client_ip: ClientRealAddr,
) -> status::Custom<Value> {
    with_context(tracing, async move {
        let pk = key_shares.signing_key.verifying_key().to_sec1_bytes();

        let pk = bytes_to_hex(pk);

        status::Custom(Status::Ok, json!(PublicKeyResponse { public_key: pk }))
    })
    .await
}
