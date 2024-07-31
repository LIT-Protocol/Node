#![allow(unused_macros)]

/// Validate that a JS variable is not empty.
macro_rules! ensure_not_empty {
    ($var:expr, $js_name:expr) => {
        anyhow::ensure!(
            !$var.is_empty(),
            deno_core::error::range_error(format!("{} must not be empty", $js_name))
        );
    };
    ($var:expr) => {
        ensure_not_empty!($var, stringify!($var))
    };
}

/// Validate that a JS variable is not blank.
macro_rules! ensure_not_blank {
    ($var:expr, $js_name:expr) => {
        anyhow::ensure!(
            !$var.trim().is_empty(),
            deno_core::error::range_error(format!("{} must not be blank", $js_name))
        );
    };
    ($var:expr) => {
        ensure_not_blank!($var, stringify!($var))
    };
}

/// Validate that a JS variable is not null or undefined.
macro_rules! ensure_not_null {
    ($var:expr, $js_name:expr) => {
        anyhow::ensure!(
            !$var.is_null(),
            deno_core::error::range_error(format!("{} must not be null or undefined", $js_name))
        );
    };
    ($var:expr) => {
        ensure_not_null!($var, stringify!($var))
    };
}

/// Validate that a JS variable matches one of the allowed values.
macro_rules! ensure_one_of {
    ($var:expr, $js_name:expr, $allowed_values:expr) => {{
        anyhow::ensure!(
            $allowed_values.iter().any(|x| *x == $var),
            deno_core::error::range_error(format!(
                "{} must be one of: {}",
                $js_name,
                $allowed_values.join(", ")
            ))
        );
    }};
    ($var:expr, $allowed_values:expr) => {
        ensure_one_of!($var, stringify!($var), $allowed_values)
    };
}

/// Validate that a JS variable is a U256 string.
macro_rules! ensure_u256 {
    ($var:expr, $js_name:expr) => {
        ensure_not_blank!($var, $js_name);
        anyhow::ensure!(
            crate::validation::is_u256_string($var),
            deno_core::error::range_error(format!("{} must be a valid U256 string", $js_name))
        );
    };
    ($var:expr) => {
        ensure_u256!($var, stringify!($var))
    };
}

// Based on https://github.com/LIT-Protocol/lit-assets/blob/a93b25d33d0d6e141818fe2d33d8bbb0b39e332f/rust/lit-core/lit-blockchain/src/util/mod.rs#L27-L37
pub(crate) fn is_u256_string<S: AsRef<str>>(s: S) -> bool {
    use ethabi::ethereum_types::U256;

    let s = s.as_ref();
    if let Some(stripped) = s.strip_prefix("0x") {
        U256::from_str_radix(stripped, 16).is_ok()
    } else {
        U256::from_dec_str(s).is_ok()
    }
}

#[allow(unused_imports)]
pub(crate) use {ensure_not_blank, ensure_not_empty, ensure_not_null, ensure_one_of, ensure_u256};
