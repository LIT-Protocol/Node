use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub org_id: String,
    pub role: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerificationRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResponse {
    pub status: bool,
    pub user_id: String,
}

#[derive(Clone)]
pub struct Jwt {
    pub header: JwtHeader,
    pub payload: JwtPayload,
    pub signature: Option<String>,
}
