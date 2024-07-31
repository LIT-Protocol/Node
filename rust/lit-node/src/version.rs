const NODE_VERSION: &str = clap::crate_version!();
const NODE_VERSION_UNMARKED: &str = "0.2.14";

pub fn get_version() -> semver::Version {
    semver::Version::parse(NODE_VERSION).expect("Failed to parse node crate version")
}

pub fn get_unmarked_version() -> semver::Version {
    semver::Version::parse(NODE_VERSION_UNMARKED).expect("Failed to parse unmarked node version")
}
