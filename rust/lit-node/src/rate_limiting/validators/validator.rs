use crate::error::Result;
use crate::rate_limiting::models::UserContext;

/// This trait is implemented by all validators that validate a rate limiting UserContext.
#[async_trait::async_trait]
pub(crate) trait Validator: Send + Sync {
    async fn validate(&self, user_context: &UserContext) -> Result<()>;
}
