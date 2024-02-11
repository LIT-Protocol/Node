pub const PKG_NAME: &str = "lit_os_core";

pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;

// constructors

generate_pkg_constructors!(PKG_NAME, pub);
