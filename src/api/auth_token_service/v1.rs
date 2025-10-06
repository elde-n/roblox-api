use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

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

pub async fn login_create(client: &mut Client) -> Result<LoginToken, Error> {
    client
        .requestor
        .request::<()>(
            Method::POST,
            &format!("{URL}/login/create"),
            None,
            None,
            None,
        )
        .await?
        .json::<LoginToken>()
        .await
}

pub async fn login_cancel(client: &mut Client, code: &str) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    client
        .requestor
        .request::<Request>(
            Method::POST,
            &format!("{URL}/login/cancel"),
            Some(&Request { code }),
            None,
            None,
        )
        .await?;

    Ok(())
}

pub async fn login_status(
    client: &mut Client,
    code: &str,
    key: &str,
) -> Result<LoginTokenStatus, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
        #[serde(rename = "privateKey")]
        key: &'a str,
    }

    client
        .requestor
        .request::<Request>(
            Method::POST,
            &format!("{URL}/login/status"),
            Some(&Request { code, key }),
            None,
            None,
        )
        .await?
        .json::<LoginTokenStatus>()
        .await
}

pub async fn inspect_code(client: &mut Client, code: &str) -> Result<InspectionInfo, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    client
        .requestor
        .request::<Request>(
            Method::POST,
            &format!("{URL}/login/enterCode"),
            Some(&Request { code }),
            None,
            None,
        )
        .await?
        .json::<InspectionInfo>()
        .await
}

pub async fn validate_code(client: &mut Client, code: &str) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    client
        .requestor
        .request::<Request>(
            Method::POST,
            &format!("{URL}/login/validateCode"),
            Some(&Request { code }),
            None,
            None,
        )
        .await?;

    Ok(())
}

pub async fn qr_code_image(client: &mut Client, key: &str, code: &str) -> Result<Vec<u8>, Error> {
    client
        .requestor
        .request::<()>(
            Method::GET,
            &format!("{URL}/login/qr-code-image"),
            None,
            Some(&[("key", key), ("code", code)]),
            None,
        )
        .await?
        .bytes()
        .await
}
