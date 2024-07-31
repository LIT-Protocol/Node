use crate::init::context::InitContext;
use crate::init::stage::oneshot::prepare::prepare_oneshot_actions;
use crate::init::stage::Outcome;
use crate::utils::vm::{
    is_amd_sev_snp, is_virtual_machine, DMESG_AMD_SEV_SNP, DMESG_HYPERVISOR_KVM,
};
use aleo_std_cpu::{get_cpu, Cpu};
use lit_core::utils::option::bool_option_to_bool;
use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{config_err, io_err, validation_err, Result};
use lit_os_core::guest::types::GuestType;
use log::{as_error, error, info};
use std::path::Path;

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    if !is_virtual_machine()? {
        error!("unable to proceed: not a VM (dmesg missing: '{}')", DMESG_HYPERVISOR_KVM);

        return Ok(Outcome::Break);
    }

    if let Err(e) = verify_tee() {
        error!(error = as_error!(e); "unable to proceed: TEE invalid");

        return Ok(Outcome::Break);
    }

    // Prepare the oneshot actions.
    prepare_oneshot_actions(ctx).await;

    match verify(ctx) {
        Err(e) => {
            error!(error = as_error!(e); "unable to proceed: context verification failed");

            Ok(Outcome::Break)
        }
        Ok(outcome) => {
            if outcome == Outcome::Continue {
                info!("Context verification: OK");
            }

            Ok(outcome)
        }
    }
}

fn verify_tee() -> Result<()> {
    match get_cpu() {
        Cpu::AMD => {
            if !is_amd_sev_snp()? {
                return Err(validation_err(
                    format!("not AMD SEV-SNP (dmesg missing: '{DMESG_AMD_SEV_SNP}')"),
                    None,
                ));
            }
        }
        v => {
            return Err(validation_err(format!("CPU type not supported: {v:?}"), None));
        }
    }

    Ok(())
}

fn verify(ctx: &InitContext) -> Result<Outcome> {
    // Verify cfg
    match ctx.cfg().litos_guest().ok() {
        None => return Err(config_err("Missing config option: lit.guest", None)),
        Some(false) => {
            return Err(config_err("Invalid config option, lit.guest must be true", None))
        }
        _ => {}
    }
    if ctx.cfg().litos_guest_allowed_cfg_keys().is_err() {
        return Err(config_err("Missing config option: lit.guest_allowed_cfg_keys", None));
    }

    // Verify build env
    // NB: root and var hashes aren't available here (they are available on the cmdline)
    ctx.build_env().verify(false, false)?;

    // Get guest type
    let guest_type = ctx
        .build_env()
        .guest_type()
        .map_err(|e| config_err(e, Some("Build is invalid: build_type is missing".into())))?;

    // Verify build options
    if !ctx.cfg().is_dev() {
        if !bool_option_to_bool(ctx.build_env().build_opt_ro.as_ref()) {
            return Err(config_err(
                format!("Build is invalid: build_opt_ro is false (env: {})", ctx.cfg().env()),
                None,
            ));
        }
        if !ctx.cfg().is_staging() {
            if bool_option_to_bool(ctx.build_env().build_opt_ssh.as_ref()) {
                return Err(config_err(
                    format!("Build is invalid: build_opt_ssh is true (env: {})", ctx.cfg().env()),
                    None,
                ));
            }
            if bool_option_to_bool(ctx.build_env().build_opt_users.as_ref()) {
                return Err(config_err(
                    format!("Build is invalid: build_opt_users is true (env: {})", ctx.cfg().env()),
                    None,
                ));
            }
        }
    }

    // Verify cmdline
    let expect_varhash = !ctx.cfg().is_dev();
    #[allow(clippy::match_like_matches_macro)]
    let expect_subnet = match guest_type {
        GuestType::Build => false,
        _ => true,
    };
    // Use release_id test directly vs ctx.is_release() as we want this to happen on Prov:bootstrap too.
    let expect_release = ctx.cmdline_env().release_id.is_some();

    ctx.cmdline_env().verify(expect_varhash, expect_subnet, expect_release)?;
    ctx.cmdline_env().matches_build_env(ctx.build_env())?;
    ctx.cmdline_env().matches_lit_cfg(ctx.cfg())?;

    // Verify disks
    let root_dev = ctx.build_env().build_luks_root_dev().ok_or(config_err(
        "expected build_luks_root_dev (based on build_luks_root_uuid) to be defined",
        None,
    ))?;
    let var_dev = ctx.build_env().build_luks_var_dev().ok_or(config_err(
        "expected build_luks_var_dev (based on build_luks_var_uuid) to be defined",
        None,
    ))?;
    let ci_dev = ctx.cfg().litos_cloud_init_dev();

    if !check_dev_exists(root_dev.as_path(), "root") {
        return Ok(Outcome::Diagnose);
    }
    if !check_dev_exists(var_dev.as_path(), "var") {
        return Ok(Outcome::Diagnose);
    }
    if !check_dev_exists(ci_dev.as_path(), "cloud-init") {
        return Ok(Outcome::Diagnose);
    }

    Ok(Outcome::Continue)
}

fn check_dev_exists(path: &Path, label: &str) -> bool {
    if !path.exists() {
        let err = io_err(format!("{label} dev ({path:?}) does not exist!"), None);

        error!(error = as_error!(err); "unable to proceed: required device missing");

        return false;
    }

    true
}
