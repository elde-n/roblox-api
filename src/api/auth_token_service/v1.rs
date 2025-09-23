use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/auth-token-service/v1";

// TODO: look into `qr-code-image`, `entercode`, `validatecode`, `metadata`

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
    let result = client
        .requestor
        .client
        .post(format!("{URL}/login/create"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<LoginToken>(response).await
}

pub async fn login_cancel(client: &mut Client, code: &str) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/login/cancel"))
        .json(&Request { code })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
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

    let result = client
        .requestor
        .client
        .post(format!("{URL}/login/status"))
        .json(&Request { code, key })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<LoginTokenStatus>(response)
        .await
}

pub async fn inspect_code(client: &mut Client, code: &str) -> Result<InspectionInfo, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/login/enterCode"))
        .json(&Request { code })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<InspectionInfo>(response)
        .await
}

pub async fn validate_code(client: &mut Client, code: &str) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/login/validateCode"))
        .json(&Request { code })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
