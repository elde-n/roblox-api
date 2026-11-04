use serde::{Deserialize, Serialize};

use crate::{DateTime, endpoint};

pub const URL: &str = "https://apis.roblox.com/auth-token-service/v1";

// TODO: look into `metadata`

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LoginStatus {
    Created,
    Validated,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginToken {
    pub code: String,
    pub status: String,
    pub private_key: String,
    pub expiration_time: DateTime,
    pub image_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginTokenStatus {
    pub status: LoginStatus,
    pub account_name: Option<String>,
    pub account_picture_url: Option<String>,
    pub expiration_time: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InspectionInfo {
    pub location: String,
    pub device_info: String,
}

#[endpoint(POST, "{URL}/login/create")]
async fn login_create() -> LoginToken {}

#[endpoint(POST, "{URL}/login/cancel")]
async fn login_cancel(code: &str) -> () {
    #[derive(Serialize)]
    struct Request<'a> {
        pub code: &'a str,
    }
    body! {
        &Request { code }
    }
    void!();
}

#[endpoint(POST, "{URL}/login/status")]
async fn login_status(code: &str, key: &str) -> LoginTokenStatus {
    #[derive(Serialize)]
    struct Request<'a> {
        pub code: &'a str,
        #[serde(rename = "privateKey")]
        pub key: &'a str,
    }
    body! {
        &Request { code, key }
    }
}

#[endpoint(POST, "{URL}/login/enterCode")]
async fn inspect_code(code: &str) -> InspectionInfo {
    #[derive(Serialize)]
    struct Request<'a> {
        pub code: &'a str,
    }
    body! {
        &Request { code }
    }
}

#[endpoint(POST, "{URL}/login/validateCode")]
async fn validate_code(code: &str) -> () {
    #[derive(Serialize)]
    struct Request<'a> {
        pub code: &'a str,
    }
    body! {
        &Request { code }
    }
    void!();
}

#[endpoint(GET, "{URL}/login/qr-code-image")]
async fn qr_code_image(key: &str, code: &str) -> Vec<u8> {
    query! {
        "key" => key,
        "code" => code,
    }
    raw_bytes!();
}
