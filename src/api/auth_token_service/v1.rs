use reqwest::{Method, Response};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

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

struct NewRequest {
    response: Response,
}

impl NewRequest {
    async fn json<T: DeserializeOwned>(self) -> Result<T, Error> {
        Ok(self.response.json::<T>().await?)
    }

    async fn bytes(self) -> Result<Vec<u8>, Error> {
        let bytes = self.response.bytes().await;
        match bytes {
            Ok(bytes) => Ok(bytes.to_vec()),
            Err(error) => Err(Error::ReqwestError(error)),
        }
    }
}

async fn new_request<'a, R: Serialize>(
    client: &mut Client,
    method: Method,
    path: &str,
    request: Option<&'a R>,
    query: Option<&'a [(&'a str, &'a str)]>,
) -> Result<NewRequest, Error> {
    let mut builder = client
        .requestor
        .client
        .request(method, format!("{URL}/{path}"))
        .headers(client.requestor.default_headers.clone());

    // Even though sending None works, it might get serialized as null in json, which is a waste of bytes
    if let Some(request) = request {
        builder = builder.json(&request);
    }

    if let Some(query) = query {
        builder = builder.query(&query);
    }

    let response = client.validate_response(builder.send().await).await?;
    Ok(NewRequest { response: response })
}

pub async fn login_create(client: &mut Client) -> Result<LoginToken, Error> {
    new_request::<()>(client, Method::POST, "login/create", None, None)
        .await?
        .json::<LoginToken>()
        .await
}

pub async fn login_cancel(client: &mut Client, code: &str) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        code: &'a str,
    }

    new_request::<Request>(
        client,
        Method::POST,
        "login/cancel",
        Some(&Request { code }),
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

    new_request::<Request>(
        client,
        Method::POST,
        "login/status",
        Some(&Request { code, key }),
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

    new_request::<Request>(
        client,
        Method::POST,
        "login/enterCode",
        Some(&Request { code }),
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

    new_request::<Request>(
        client,
        Method::POST,
        "login/validateCode",
        Some(&Request { code }),
        None,
    )
    .await?;

    Ok(())
}

pub async fn qr_code_image(client: &mut Client, key: &str, code: &str) -> Result<Vec<u8>, Error> {
    new_request::<()>(
        client,
        Method::GET,
        "login/qr-code-image",
        None,
        Some(&[("key", key), ("code", code)]),
    )
    .await?
    .bytes()
    .await
}
