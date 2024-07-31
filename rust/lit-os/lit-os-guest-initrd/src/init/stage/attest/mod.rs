use lit_attestation::verification::Policy;
use log::{as_error, error, info};

use lit_os_core::error::{validation_err, Result};
use lit_os_core::guest::oneshot::config::ACTION_TYPE_BOOTSTRAP;

use crate::init::context::InitContext;
use crate::init::stage::Outcome;

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    if let Err(e) = verify_attestation(ctx).await {
        error!(error = as_error!(e); "Attestation failed");

        return Ok(Outcome::Diagnose);
    }

    Ok(Outcome::Continue)
}

pub async fn verify_attestation(ctx: &mut InitContext) -> Result<()> {
    // Request (and verify) the attestation.
    // NB: Unable to sign here as we do not have access to the build key.
    let attestation = ctx.request_attestation(None, None).await?;

    if ctx.is_release() {
        info!("Verifying attestation & release (may take some time to download certs) against SelfVerify");
        attestation
            .verify_full(ctx.cfg(), None, Some(Policy::SelfVerify))
            .await
            .map_err(|e| validation_err(e, Some("failed to self-verify".into())))?;
    } else {
        // Safe boot, avoid network IO.
        if !ctx.is_safe_boot() && !ctx.has_oneshot_action(ACTION_TYPE_BOOTSTRAP) {
            info!("Verifying attestation (may take some time to download certs) against SEV-SNP");
            attestation
                .verify()
                .await
                .map_err(|e| validation_err(e, Some("failed to self-verify".into())))?;
        }
    }
    info!("Attestation verified");

    Ok(())
}
