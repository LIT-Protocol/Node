use crate::error::Result;
use siwe::Message;

use self::session_capability_object::SessionCapabilityObject;

pub mod recap;
pub mod session_capability_object;

pub(crate) fn extract_and_verify(siwe_message: Message) -> Result<impl SessionCapabilityObject> {
    recap::extract_and_verify(siwe_message)
}
