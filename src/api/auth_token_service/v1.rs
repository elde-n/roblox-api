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

endpoint! {
    login_create() -> LoginToken {
        POST "{URL}/login/create";
    }

    login_cancel(code: &str) -> () {
        POST "{URL}/login/cancel";
        types {
            Request<'a> {
                code: &'a str,
            }
        }
        body_serialize {
            &Request { code }
        }
    }

    login_status(code: &str, key: &str) -> LoginTokenStatus {
        POST "{URL}/login/status";
        types {
            Request<'a> {
                code: &'a str,
                key("privateKey"): &'a str,
            }
        }
        body_serialize {
            &Request { code, key }
        }
    }

    inspect_code(code: &str) -> InspectionInfo {
        POST "{URL}/login/enterCode";
        types {
            Request<'a> {
                code: &'a str,
            }
        }
        body_serialize {
            &Request { code }
        }
    }

    validate_code(code: &str) -> () {
        POST "{URL}/login/validateCode";
        types {
            Request<'a> {
                code: &'a str,
            }
        }
        body_serialize {
            &Request { code }
        }
    }

    qr_code_image(key: &str, code: &str) -> Vec<u8> {
        GET "{URL}/login/qr-code-image";
        query {
            "key" => key,
            "code" => code,
        }
        raw_bytes
    }
}
