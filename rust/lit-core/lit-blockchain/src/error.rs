pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;

pub const PKG_NAME: &str = "lit_blockchain";

// constructors

generate_pkg_constructors!(PKG_NAME);
