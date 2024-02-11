use lit_os_core::guest::env::build::GuestBuildEnv;

pub(crate) trait GuestTemplateHelper {
    /// Verify whether an instance is valid (or considered dirty/invalid).
    fn is_valid(&self) -> bool;
}

impl GuestTemplateHelper for GuestBuildEnv {
    fn is_valid(&self) -> bool {
        true
    }
}
