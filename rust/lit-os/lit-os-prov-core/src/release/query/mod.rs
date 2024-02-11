use std::collections::HashMap;

use ethers::prelude::*;

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseType};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::release::release_id_to_string;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_32_are_zeros;
use lit_os_core::guest::types::GuestType;

use crate::error::{blockchain_err, Result};
use crate::release::common::load::load_release;
use crate::release::common::types::Release;
use crate::release::query::types::QueryReleaseRequest;

pub mod types;

pub async fn query_releases(
    cfg: &LitConfig, resolver: &ContractResolver, req: &QueryReleaseRequest,
) -> Result<HashMap<GuestType, Release>> {
    // Verify
    req.verify()?;

    // Setup
    let contract = resolver.release_register_contract(cfg).await?;
    let platform: ReleasePlatform = (*req.vcpu_type()).into();

    let mut results = HashMap::new();

    for typ in req.types() {
        let release_type: ReleaseType = typ.typ().into();

        let release_id = contract
            .get_active_release(
                *req.env() as u8,
                release_type as u8,
                Bytes::from(typ.kind_to_vec()),
                platform as u8,
            )
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        if bytes_32_are_zeros(&release_id) {
            continue;
        }

        let release_id = release_id_to_string(&release_id);
        if let Ok(release) = load_release(cfg, resolver, &release_id, true).await {
            results.insert(typ.typ(), release);
        }
    }

    Ok(results)
}
