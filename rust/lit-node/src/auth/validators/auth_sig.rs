use crate::auth::auth_material::JsonAuthSig;
use crate::auth::resources::LitResourceAbility;
use crate::error::Result;

/// This trait is implemented by all validators that validate a SessionSig.
#[async_trait::async_trait]
pub(crate) trait SessionSigAuthSigValidator: Send + Sync {
    /// Basic validation of the auth sig without checking capabilities to perform
    /// the requested resource ability.
    async fn validate_auth_sig_basic(
        &self,
        auth_sig: &JsonAuthSig,
        session_pubkey: &str,
    ) -> Result<()>;

    /// Validation of the auth sig AND checking capabilities to perform
    /// the requested resource ability.
    async fn validate_auth_sig(
        &self,
        auth_sig: &JsonAuthSig,
        session_pubkey: &str,
        requested_lit_resource_ability: &LitResourceAbility,
    ) -> Result<()>;
}
