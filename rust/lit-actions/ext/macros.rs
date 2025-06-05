#![allow(unused_macros)]

// Macro to simplify implementing synchronous ops over gRPC.
macro_rules! remote_op {
    ($name:ident, $state:ident, $send:expr, $match:pat => $res:expr) => {{
        let error_box = |msg: String| -> ::deno_error::JsErrorBox {
            ::deno_error::JsErrorBox::generic(msg)
        };

        let _: &mut ::deno_core::OpState = $state;
        let (tx, rx) = {
            let tx = $state
                .borrow::<::flume::Sender<::tonic::Result<::lit_actions_grpc::proto::ExecuteJsResponse>>>()
                .clone();
            let rx = $state.borrow::<::flume::Receiver<::lit_actions_grpc::proto::ExecuteJsRequest>>().clone();
            (tx, rx)
        };

        let op_name = stringify!($name);
        tx.send(Ok($send.into()))
            .map_err(|e| error_box(format!("{op_name}: {e}")))?;

        let resp = rx
            .recv()
            .map_err(|e| error_box(format!("{op_name}: {e}")))?;

        match resp.union {
            Some($match) => $res,
            Some(::lit_actions_grpc::proto::UnionRequest::ReportError(
                ::lit_actions_grpc::proto::ErrorResponse { error }
            )) => {
                Err(error_box(error))
            }
            other => Err(error_box(format!(
                "{op_name}: unexpected response: {other:?}"
            ))),
        }
    }};
}

// Macro to simplify implementing asynchronous ops over gRPC.
macro_rules! remote_op_async {
    ($name:ident, $state:ident, $send:expr, $match:pat => $res:expr) => {{
        let error_box = |msg: String| -> ::deno_error::JsErrorBox {
            ::deno_error::JsErrorBox::generic(msg)
        };

        let _: ::std::rc::Rc<::std::cell::RefCell<::deno_core::OpState>> = $state;
        let (tx, rx) = {
            let state = $state.borrow();
            let tx = state
                .borrow::<::flume::Sender<::tonic::Result<::lit_actions_grpc::proto::ExecuteJsResponse>>>()
                .clone();
            let rx = state
                .borrow::<::flume::Receiver<::lit_actions_grpc::proto::ExecuteJsRequest>>()
                .clone();
            (tx, rx)
        };

        let op_name = stringify!($name);
        tx.send_async(Ok($send.into()))
            .await
            .map_err(|e| error_box(format!("{op_name}: {e}")))?;

        // Must be blocking to preserve ops order
        let resp = rx
            .recv()
            .map_err(|e| error_box(format!("{op_name}: {e}")))?;

        match resp.union {
            Some($match) => $res,
            Some(::lit_actions_grpc::proto::UnionRequest::ReportError(
                ::lit_actions_grpc::proto::ErrorResponse { error }
            )) => {
                Err(error_box(error))
            }
            other => Err(error_box(format!(
                "{op_name}: unexpected response: {other:?}"
            ))),
        }
    }};
}

// Note: For all ensure_* macros, the surrounding function's return value
// is required to be Result<_, deno_error::JsErrorBox>.

/// Validate that a JS variable is not empty.
macro_rules! ensure_not_empty {
    ($var:expr, $js_name:expr) => {
        if $var.is_empty() {
            return Err(::deno_error::JsErrorBox::range_error(format!(
                "{} must not be empty",
                $js_name
            )));
        }
    };
    ($var:expr) => {
        ensure_not_empty!($var, stringify!($var))
    };
}

/// Validate that a JS variable is not blank.
macro_rules! ensure_not_blank {
    ($var:expr, $js_name:expr) => {
        if $var.trim().is_empty() {
            return Err(::deno_error::JsErrorBox::range_error(format!(
                "{} must not be blank",
                $js_name
            )));
        }
    };
    ($var:expr) => {
        ensure_not_blank!($var, stringify!($var))
    };
}

/// Validate that a JS variable is not null or undefined.
macro_rules! ensure_not_null {
    ($var:expr, $js_name:expr) => {
        if $var.is_null() {
            return Err(::deno_error::JsErrorBox::range_error(format!(
                "{} must not be null or undefined",
                $js_name
            )));
        }
    };
    ($var:expr) => {
        ensure_not_null!($var, stringify!($var))
    };
}

/// Validate that a JS variable matches one of the allowed values.
macro_rules! ensure_one_of {
    ($var:expr, $js_name:expr, $allowed_values:expr) => {{
        if !$allowed_values.iter().any(|x| *x == $var) {
            return Err(::deno_error::JsErrorBox::range_error(format!(
                "{} must be one of: {}",
                $js_name,
                $allowed_values.join(", ")
            )));
        }
    }};
    ($var:expr, $allowed_values:expr) => {
        ensure_one_of!($var, stringify!($var), $allowed_values)
    };
}

/// Validate that a JS variable is a U256 string.
macro_rules! ensure_u256 {
    ($var:expr, $js_name:expr) => {
        ensure_not_blank!($var, $js_name);
        if !crate::macros::is_u256_string($var) {
            return Err(::deno_error::JsErrorBox::range_error(format!(
                "{} must be a valid U256 string",
                $js_name
            )));
        };
    };
    ($var:expr) => {
        ensure_u256!($var, stringify!($var))
    };
}

// Based on https://github.com/LIT-Protocol/lit-assets/blob/a93b25d33d0d6e141818fe2d33d8bbb0b39e332f/rust/lit-core/lit-blockchain/src/util/mod.rs#L27-L37
pub(crate) fn is_u256_string<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();
    if let Some(stripped) = s.strip_prefix("0x") {
        ::ethabi::ethereum_types::U256::from_str_radix(stripped, 16).is_ok()
    } else {
        ::ethabi::ethereum_types::U256::from_dec_str(s).is_ok()
    }
}

#[allow(unused_imports)]
pub(crate) use {
    ensure_not_blank, ensure_not_empty, ensure_not_null, ensure_one_of, ensure_u256, remote_op,
    remote_op_async,
};
