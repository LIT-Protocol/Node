#![allow(clippy::unwrap_used, clippy::ignored_unit_patterns)]
tonic::include_proto!("com.litprotocol.actions");

pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("lit_actions_descriptor");

// The following makes the generated code more ergonomic to use

pub use action_client::ActionClient;
pub use action_server::{Action, ActionServer};

pub use execute_js_request::ErrorResponse;
pub use execute_js_request::{ExecutionRequest, Union as UnionRequest};
pub use execute_js_response::{ExecutionResult, Union as UnionResponse};

impl From<&str> for ExecutionRequest {
    fn from(code: &str) -> Self {
        Self {
            code: code.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for ExecutionRequest {
    fn from(code: String) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }
}

impl From<ExecutionRequest> for ExecuteJsRequest {
    fn from(req: ExecutionRequest) -> Self {
        Self {
            union: Some(UnionRequest::Execute(req)),
        }
    }
}

impl From<ExecutionResult> for ExecuteJsResponse {
    fn from(res: ExecutionResult) -> Self {
        Self {
            union: Some(UnionResponse::Result(res)),
        }
    }
}

impl From<ErrorResponse> for ExecuteJsRequest {
    fn from(req: ErrorResponse) -> Self {
        Self {
            union: Some(UnionRequest::ReportError(req)),
        }
    }
}

// A wrapper for ExecutionRequest with a custom Debug impl
pub struct DebugExecutionRequest<'a>(&'a ExecutionRequest);

impl<'a> From<&'a ExecutionRequest> for DebugExecutionRequest<'a> {
    fn from(req: &'a ExecutionRequest) -> Self {
        Self(req)
    }
}

impl std::fmt::Debug for DebugExecutionRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MAX_CODE_LEN: usize = 500;

        let Self(req) = self;

        let truncated_code = if req.code.len() > MAX_CODE_LEN {
            &format!(
                "{}... (truncated, {} bytes total)",
                &req.code[..MAX_CODE_LEN],
                req.code.len()
            )
        } else {
            req.code.as_str()
        };

        f.debug_struct("ExecutionRequest")
            .field("code", &truncated_code)
            .field("js_params", &req.js_params)
            .field("auth_context", &req.auth_context)
            .field("timeout", &req.timeout)
            .field("memory_limit", &req.memory_limit)
            .field("http_headers", &req.http_headers)
            .finish()
    }
}

// Declare op request/response types
// For example, decl_op!(Print) will declare PrintRequest and PrintResponse
// as well as conversions to and from ExecuteJsRequest and ExecuteJsResponse
macro_rules! decl_op {
    ($prefix:ident) => {
        concat_idents::concat_idents!(typ = $prefix, Response {
            pub use execute_js_request::typ;
            impl From<typ> for ExecuteJsRequest {
                fn from(resp: typ) -> Self {
                    Self {
                        union: Some(UnionRequest::$prefix(resp)),
                    }
                }
            }
        });
        concat_idents::concat_idents!(typ = $prefix, Request {
            pub use execute_js_response::typ;
            impl From<typ> for ExecuteJsResponse {
                fn from(req: typ) -> Self {
                    Self {
                        union: Some(UnionResponse::$prefix(req)),
                    }
                }
            }
        });
    };
}

decl_op!(AesDecrypt);
decl_op!(CallChild);
decl_op!(CallContract);
decl_op!(CheckConditions);
decl_op!(ClaimKeyIdentifier);
decl_op!(GetLatestNonce);
decl_op!(IncrementFetchCount);
decl_op!(PkpPermissionsGetPermitted);
decl_op!(PkpPermissionsGetPermittedAuthMethodScopes);
decl_op!(PkpPermissionsIsPermitted);
decl_op!(PkpPermissionsIsPermittedAuthMethod);
decl_op!(Print);
decl_op!(PubkeyToTokenId);
decl_op!(SetResponse);
decl_op!(SignEcdsa);
decl_op!(BroadcastAndCollect);
decl_op!(DecryptAndCombine);
decl_op!(SignAndCombineEcdsa);
decl_op!(GetRpcUrl);
decl_op!(P2pBroadcast);
decl_op!(P2pCollectFromLeader);
decl_op!(IsLeader);
decl_op!(EncryptBls);
decl_op!(DecryptToSingleNode);
