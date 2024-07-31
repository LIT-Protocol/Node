use std::fmt::Debug;

use crate::auth::resources::LitResourceAbility;
use crate::error::Result;

pub trait SessionCapabilityObject: Debug {
    fn verify_capabilities_for_resource(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
    ) -> Result<()>;
}
