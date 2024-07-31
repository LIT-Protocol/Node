use std::fmt::Debug;

use crate::models::auth::LitResourcePrefix;

pub trait LitResource: Debug {
    /// Get the fully qualified IRI for this resource. This is compatible with the URI spec
    /// outlined here: https://datatracker.ietf.org/doc/html/rfc3986.
    fn get_resource_key(&self) -> String {
        format!(
            "{}://{}",
            self.get_resource_prefix(),
            self.get_resource_id()
        )
    }

    /// Get the identifier for this resource.
    fn get_resource_id(&self) -> &String;

    /// Get the prefix for this resource.
    fn get_resource_prefix(&self) -> LitResourcePrefix;
}
