use std::str::FromStr;

use libsecp256k1::PublicKey;
use log::{as_serde, trace};
use serde_json::Value;

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseStatus, ReleaseType};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::release::{subnet_id_from_release_id, RELEASE_ID_STR_LEN};
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::error::Error;
use lit_core::utils::binary::bytes_to_hex;
pub use policy::*;

use crate::attestation::{AdminSignedType, Attestation, AttestationType, DATA_KEY_RELEASE_ID};
use crate::error::{
    attestation_err_code, conversion_err, conversion_err_code, unexpected_err_code,
    validation_err_code, Result, EC,
};
use crate::utils::sev_snp::FamilyIdBuilder;
use crate::verification::blockchain::{
    get_release, get_release_register_contract, get_staking_balances_contract,
    get_staking_contract, release_has_allowed_admin_signing_public_key,
    release_has_allowed_author_key_digest, staking_has_allowed_node_signing_public_key,
    unwrap_or_create_contract_resolver,
};
use crate::AmdSevSnpAttestation;

mod blockchain;
mod cache;
pub mod policy;

/// Additional verification to compare with blockchain.
pub async fn verify_full(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, data: &Attestation,
    policy: Option<Box<dyn VerificationPolicy>>,
) -> Result<Option<PublicKey>> {
    trace!("verify_full()");
    let policy = policy.unwrap_or(Box::new(Policy::Default) as Box<dyn VerificationPolicy>);

    // Perform base data validation
    data.verify().await?;

    // Perform on-chain validation
    return match data.typ() {
        AttestationType::AmdSevSnp => verify_amd_sev_snp(cfg, resolver, data, policy.as_ref())
            .await
            .map_err(|e| err_add_fields(e, data, policy.as_ref())),
        AttestationType::AdminSigned => {
            if let Some(allowed) = policy.allowed_attestation_types() {
                if !allowed.contains(data.typ()) {
                    return Err(err_add_fields(validation_err_code(
                        "attestation verify full failed due to policy: type not in allowed_attestation_types",
                        EC::AttestationPolicyVerifyFailed,
                        None,
                    )
                      .add_detail(format!(
                          "Policy forbids 'AttestationType: {}' (reason: allowed_attestation_types)",
                          AttestationType::AdminSigned
                      )), data, policy.as_ref()));
                }
            }
            if let Some(allowed) = policy.require_signed() {
                Ok(Some(
                    verify_admin_signed(cfg, resolver, data, policy.as_ref(), allowed)
                        .await
                        .map_err(|e| err_add_fields(e, data, policy.as_ref()))?,
                ))
            } else {
                Err(err_add_fields(validation_err_code(
                    "attestation verify full failed due to policy: require_admin_signed is none",
                    EC::AttestationPolicyVerifyFailed,
                    None,
                )
               .add_detail(format!(
                   "Policy forbids 'AttestationType: {}' (reason: require_admin_signed)",
                   AttestationType::AdminSigned
               )), data, policy.as_ref()))
            }
        }
    };
}

pub(crate) async fn verify_amd_sev_snp(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, data: &Attestation,
    policy: &dyn VerificationPolicy,
) -> Result<Option<PublicKey>> {
    verify_release(cfg, resolver, data, policy).await
}

pub(crate) async fn verify_admin_signed(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, data: &Attestation,
    policy: &dyn VerificationPolicy, allowed: Vec<AdminSignedType>,
) -> Result<PublicKey> {
    match verify_signatures(cfg, resolver, data, policy, allowed, None, "verify_admin_signed").await
    {
        Ok((passed_type, public_key)) => {
            trace!(
                type = as_serde!(data.typ()),
                auth_type = as_serde!(passed_type),
                auth_public_key = as_serde!(bytes_to_hex(public_key.serialize()));
                "Attestation verify OK"
            );

            Ok(public_key)
        }
        Err(e) => Err(e),
    }
}

pub(crate) async fn verify_signatures(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, data: &Attestation,
    _policy: &dyn VerificationPolicy, allowed: Vec<AdminSignedType>,
    release_type: Option<ReleaseType>, debug_label: &str,
) -> Result<(AdminSignedType, PublicKey)> {
    let resolver = unwrap_or_create_contract_resolver(cfg, resolver, None, debug_label).await?;

    let mut passed_public_key: Option<PublicKey> = None;
    let mut passed_type: Option<AdminSignedType> = None;

    let register_contract = get_release_register_contract(cfg, &resolver, debug_label).await?;
    let staking_contract = get_staking_contract(cfg, &resolver, debug_label).await?;
    let staking_balances_contract =
        get_staking_balances_contract(cfg, &resolver, debug_label).await?;

    let mut public_keys = data.public_keys()?;
    if !public_keys.is_empty() {
        if data.is_facility_guest_service() {
            // Remove the last (build key signature)
            let _ = public_keys.pop();
        }

        // Check signatures one by one from the end.
        for public_key in public_keys.iter().rev() {
            let public_key_vec = public_key.serialize().to_vec();

            for at in allowed.iter() {
                match *at {
                    AdminSignedType::Admin => {
                        if release_has_allowed_admin_signing_public_key(
                            &register_contract,
                            resolver.subnet_id().as_str(),
                            public_key_vec.clone(),
                            debug_label,
                        )
                        .await?
                        {
                            passed_type = Some(*at);
                            passed_public_key = Some(*public_key);
                            break;
                        }
                    }
                    AdminSignedType::Operator => {
                        if matches!(release_type, None | Some(ReleaseType::Node))
                            && staking_has_allowed_node_signing_public_key(
                                &staking_contract,
                                &staking_balances_contract,
                                resolver.subnet_id().as_str(),
                                public_key_vec.as_ref(),
                                debug_label,
                            )
                            .await?
                        {
                            passed_type = Some(*at);
                            passed_public_key = Some(*public_key);
                            break;
                        }
                    }
                }
            }

            if passed_type.is_some() {
                break;
            }
        }
    }

    match (passed_type, passed_public_key) {
        (Some(passed_type), Some(public_key)) => {
            Ok((passed_type, public_key))
        }
        _ => {
            Err(validation_err_code(
                format!("{debug_label}: signature validation failed for public keys (not listed on-chain as any kind of admin capable of signing)"),
                EC::AttestationPolicyVerifyFailed,
                None
            )
                .add_field("allowed_types", Value::Array(allowed.iter().map(|v| Value::String(v.to_string())).collect()))
                .add_detail("Admin signature or records are not recognised"))
        }
    }
}

pub(crate) async fn verify_release(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, data: &Attestation,
    policy: &dyn VerificationPolicy,
) -> Result<Option<PublicKey>> {
    let debug_label = "verify_release";

    // Verify facility
    if policy.require_facility_service() && !data.is_facility_guest_service() {
        return Err(validation_err_code(
            "attestation not from guest service (with policy: require_facility_service)",
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail("Policy requires attestation originate from within a secure guest"));
    }

    // Extract and verify release id
    let release_id = data.release_id().ok_or_else(|| {
        validation_err_code(
            format!("missing required data key: {DATA_KEY_RELEASE_ID}"),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail("Release Id in data is invalid")
    })?;
    let release_id_padded = data.release_id_as_b32_slice().ok_or_else(|| {
        validation_err_code(
            format!("failed to zero pad release id: {release_id:?}"),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail("Release Id in data is invalid")
    })?;

    // Verify subnet id
    if release_id.len() != RELEASE_ID_STR_LEN {
        return Err(validation_err_code(
            format!(
                "release id is not of the right length: ({} vs {})",
                release_id.len(),
                RELEASE_ID_STR_LEN
            ),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail("Release Id in data is invalid"));
    }

    let release_subnet_id = subnet_id_from_release_id(&release_id)
        .map_err(|e| validation_err_code(e, EC::AttestationPolicyVerifyFailed, None))?;
    let our_subnet_id = cfg.subnet_id()?;

    if !policy.allow_subnet(&our_subnet_id, &release_subnet_id) {
        return Err(validation_err_code(
            "Policy forbids requests from this subnet",
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_source_to_details()
        .add_field("our_subnet_id", Value::from(our_subnet_id.clone()))
        .add_field("other_subnet_id", Value::from(release_subnet_id.clone())));
    }

    // Create a resolver based on the release
    let release_resolver = unwrap_or_create_contract_resolver(
        cfg,
        resolver,
        Some(&release_subnet_id),
        "verify_release",
    )
    .await?;

    // Verify release exists
    let contract = get_release_register_contract(cfg, &release_resolver, debug_label).await?;
    let release =
        get_release(&contract, release_subnet_id.as_str(), release_id_padded, debug_label).await?;

    let release_status = ReleaseStatus::try_from(release.status).map_err(|e| {
        conversion_err_code(
            e,
            EC::AttestationPolicyVerifyFailed,
            Some(format!("unable to convert status from release: {release_id}")),
        )
    })?;

    if release_status == ReleaseStatus::Null {
        return Err(validation_err_code(
            format!("release id ({release_id}) not found on chain"),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail(format!("Release Id ({release_id}) not found")));
    }
    if release_status != ReleaseStatus::Active {
        return Err(validation_err_code(
            format!("release id ({release_id}) is not active (status: {release_status})"),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail(format!("Release Id ({release_id}) is not active")));
    }

    // Verify environment matches
    let release_env = LitEnv::try_from(release.env).map_err(|e| {
        conversion_err_code(
            e,
            EC::AttestationPolicyVerifyFailed,
            Some(format!(
                "unable load release env as LitEnv, release_id: {release_id}, release env: {}",
                release.env
            )),
        )
    })?;
    if !policy.allow_env(cfg.env(), &release_env) {
        return Err(validation_err_code(
            format!(
                "release ({}) env ({}) forbidden by policy (our env: {})",
                release_id,
                release_env,
                cfg.env()
            ),
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail(format!("Release Id ({release_id}) env denied by policy")));
    }

    let mut build_public_key_vec: Option<Vec<u8>> = None;
    if !data.signatures().is_empty() {
        // Verify build key / public key
        let build_public_key = data.last_public_key().map_err(|e| {
            attestation_err_code(
                e,
                EC::AttestationPolicyVerifyFailed,
                Some("failed to get last_public_key".into()),
            )
        })?;
        let build_public_key_bytes = build_public_key.serialize();
        let build_public_key_bytes = build_public_key_bytes.to_vec();

        if !release.public_key.eq(&build_public_key_bytes) {
            return Err(validation_err_code(
                format!(
                    "release ({}) build public key is invalid ({})",
                    release_id,
                    bytes_to_hex(&build_public_key_bytes)
                ),
                EC::AttestationPolicyVerifyFailed,
                None,
            )
            .add_detail(format!("Release Id ({release_id}) public key is invalid")));
        }

        build_public_key_vec = Some(build_public_key_bytes);
    }

    // Verify release type
    let release_type = ReleaseType::try_from(release.typ)?;
    if let Ok(our_type) = cfg.config().get_string("build.type") {
        let our_type = ReleaseType::from_str(our_type.as_str())?;

        if !policy.allow_type(&our_type, &release_type) {
            return Err(validation_err_code(
                "Policy forbids requests from this type",
                EC::AttestationPolicyVerifyFailed,
                None,
            )
            .add_source_to_details()
            .add_field("our_type", Value::from(our_type.to_string()))
            .add_field("other_type", Value::from(release_type.to_string())));
        }
    } else {
        return Err(validation_err_code(
            "Policy forbids requests from this type (our type is unknown)",
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_source_to_details());
    }

    // Platform specific
    let release_platform = ReleasePlatform::try_from(release.platform).map_err(|e| {
        conversion_err(e, Some(format!("unable to convert platform from release: {release_id}")))
    })?;

    // Validate the signatures
    let mut auth_type: Option<AdminSignedType> = None;
    let mut auth_public_key: Option<PublicKey> = None;
    if let Some(allowed) = policy.require_signed() {
        let (a_type, a_pk) = verify_signatures(
            cfg,
            Some(&release_resolver),
            data,
            policy,
            allowed,
            Some(release_type),
            debug_label,
        )
        .await
        .map_err(|e| err_add_fields(e, data, policy))?;

        let _ = auth_type.insert(a_type);
        let _ = auth_public_key.insert(a_pk);
    } else if data.signatures().is_empty() && !policy.allow_unsigned() {
        return Err(validation_err_code(
            "attestation has no signatures",
            EC::AttestationPolicyVerifyFailed,
            None,
        )
        .add_detail("Attestation has no singatures (policy forbids unsigned)"));
    }

    match data.typ() {
        AttestationType::AmdSevSnp => {
            // Verify platform
            #[allow(unreachable_patterns)]
            match release_platform {
                ReleasePlatform::MetalAmdSev => {} // Ok
                _ => {
                    return Err(validation_err_code(
                        format!(
                            "release ({}) platform ({}) isn't valid for attestation type ({:?})",
                            release_id,
                            release_platform,
                            AttestationType::AmdSevSnp
                        ),
                        EC::AttestationPolicyVerifyFailed,
                        None,
                    )
                    .add_detail(format!("Release Id ({release_id}) platform denied by policy")));
                }
            }

            let report = data.report()?;

            // Verify id digest
            let id_key_digest = release.id_key_digest.to_vec();
            if !report.id_key_digest.eq(&id_key_digest) {
                return Err(validation_err_code(
                    format!(
                        "release ({}) id key digest is invalid ({})",
                        release_id,
                        bytes_to_hex(&id_key_digest)
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }

            // Verify author key digest
            if !release_has_allowed_author_key_digest(
                &contract,
                release_subnet_id.as_str(),
                report.author_key_digest.clone(),
                debug_label,
            )
            .await?
            {
                return Err(validation_err_code(
                    format!(
                        "release ({}) author key digest is invalid ({})",
                        release_id,
                        bytes_to_hex(&report.author_key_digest)
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }

            // Verify image_id
            let image_id_str = bytes_to_hex(&report.image_id.0[..]);
            if !release_id.starts_with(&image_id_str) {
                return Err(validation_err_code(
                    format!("release ({release_id}) image_id is invalid ({image_id_str})"),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }

            // Verify family_id
            let family_id = FamilyIdBuilder::try_from(report.family_id)?;
            if !(family_id.env as u8).eq(&release.env) {
                return Err(validation_err_code(
                    format!(
                        "release ({}) family_id env is invalid ({})",
                        release_id, family_id.env
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }
            if !(family_id.typ as u8).eq(&release.typ) {
                return Err(validation_err_code(
                    format!(
                        "release ({}) family_id type is invalid ({})",
                        release_id, family_id.typ
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }
            if !(family_id.platform as u8).eq(&release.platform) {
                return Err(validation_err_code(
                    format!(
                        "release ({}) family_id platform is invalid ({})",
                        release_id, family_id.platform
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }
            if !family_id.guest_options.eq(&(release.options.as_u32() as u8)) {
                return Err(validation_err_code(
                    format!(
                        "release ({}) family_id guest_options are invalid ({})",
                        release_id, family_id.guest_options
                    ),
                    EC::AttestationPolicyVerifyFailed,
                    None,
                ));
            }

            trace!(
                type = as_serde!(data.typ().to_string()),
                auth_type = as_serde!(auth_type.map(|v| v.to_string()).unwrap_or_default()),
                auth_public_key = as_serde!(auth_public_key.map(|v| bytes_to_hex(v.serialize())).unwrap_or_default()),
                build_public_key = as_serde!(build_public_key_vec.map(bytes_to_hex).unwrap_or_default()),
                release_id = as_serde!(&release_id),
                image_id = as_serde!(report.image_id.hex()),
                family_id = as_serde!(report.family_id.hex()),
                family_id_fp = as_serde!(bytes_to_hex(family_id.fingerprint)),
                family_id_seed = as_serde!(bytes_to_hex(family_id.seed));
                "Attestation verify OK"
            );
        }
        typ => {
            return Err(unexpected_err_code(
                format!("verify_release not implemented for: {typ}"),
                EC::AttestationPolicyVerifyFailed,
                None,
            )
            .add_detail("Attestation type not implemented correctly"));
        }
    }

    Ok(auth_public_key)
}

// Common
pub(crate) fn err_add_fields(
    mut err: Error, data: &Attestation, policy: &dyn VerificationPolicy,
) -> Error {
    err = err
        .add_field("type", Value::from(data.typ().to_string()))
        .add_field("policy", Value::String(policy.name()));

    if let Ok(public_keys) = data.public_keys() {
        err = err.add_field(
            "public_keys",
            Value::Array(
                public_keys.iter().map(|v| Value::String(bytes_to_hex(v.serialize()))).collect(),
            ),
        );
    }

    if let Some(release_id) = data.release_id() {
        err = err.add_field("release_id", Value::from(release_id.clone()));

        if let Ok(subnet_id) = subnet_id_from_release_id(&release_id) {
            err = err.add_field("release_subnet_id", Value::from(subnet_id));
        }
    }

    #[allow(clippy::single_match)]
    match data.typ() {
        AttestationType::AmdSevSnp => {
            if let Ok(report) = data.report() {
                err = err
                    .add_field("image_id", Value::from(report.image_id.hex()))
                    .add_field("family_id", Value::from(report.family_id.hex()));

                if let Ok(family_id) = FamilyIdBuilder::try_from(report.family_id) {
                    err = err
                        .add_field("family_id_fp", Value::from(bytes_to_hex(family_id.fingerprint)))
                        .add_field("family_id_seed", Value::from(bytes_to_hex(family_id.seed)));
                }
            }
        }
        _ => {}
    }

    err
}
