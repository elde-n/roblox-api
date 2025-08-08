use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/auth-token-service/v1";

// TODO: look into `qr-code-image`, `entercode`, `validatecode`, `metadata`

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum LoginStatus {
    Created,
    Validated,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LoginToken {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "expirationTime")]
    pub expiration_time: DateTime,
    #[serde(rename = "imagePath")]
    pub image_path: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LoginTokenStatus {
    #[serde(rename = "status")]
    pub status: LoginStatus,
    #[serde(rename = "accountName")]
    pub account_name: Option<String>,
    #[serde(rename = "accountPictureUrl")]
    pub account_picture_url: Option<String>,
    #[serde(rename = "expirationTime")]
    pub expiration_time: String,
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
