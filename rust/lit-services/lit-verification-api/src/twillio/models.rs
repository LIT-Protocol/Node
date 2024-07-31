use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EmailConfiguration {
    from: Option<String>,
    from_name: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct OtpStartRequest {
    pub otp: String,
    /**
        Used if the otp is a phone number
        for the calling code
    */
    pub country: Option<u32>,
    pub request_id: String,

    /**
     * properties for configuring email sender
     */
    pub email_configuration: Option<EmailConfiguration>,

    /**
     * Overrides the service name with a user defined value
     */
    pub custom_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OtpCheckRequest {
    pub otp: String,
    pub code: String,
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct OtpStartResponse {
    pub message: Option<String>,
    pub callback: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OtpCheckResponse {
    pub message: Option<String>,
    pub status: Option<String>,
    pub token_jwt: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RequestOptions {
    pub user_id: String,
    pub code: Option<String>,
    pub email_configuration: Option<EmailConfiguration>,
    pub custom_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwillioStartVerificationResponse {
    pub sid: String,
    pub to: String,
    pub status: String,
    pub valid: bool,
    pub date_created: String,
    pub date_updated: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwillioCheckVerificationResponse {
    pub sid: String,
    pub to: String,
    pub status: String,
    pub valid: bool,
    pub date_created: String,
    pub date_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCheckRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCheckResponse {
    pub is_used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwillioVerificationCheckResponse {
    pub send_code_attempts: u64,
    pub channel: String,
    pub status: String,
    pub to: String,
}
