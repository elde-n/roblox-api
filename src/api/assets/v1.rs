use reqwest::{
    header::{self, HeaderValue},
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, DateTime, Error, client::Client, endpoint};

pub const URL: &str = "https://apis.roblox.com/assets/user-auth/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Creator {
    // TODO: cast to u64
    UserId(String), // can be sent as a u64, but it's returned as a string in some cases
    GroupId(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreationContext {
    pub creator: Creator,
    pub expected_price: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModerationResult {
    #[serde(rename = "moderationState")]
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    #[serde(rename = "assetId")]
    pub id: String,
    pub icon: Option<String>,
    #[serde(rename = "displayName")]
    pub name: String,
    pub description: String,

    pub path: String,
    pub state: String,
    pub asset_type: AssetTypeId,

    pub revision_id: String,
    #[serde(rename = "revisionCreateTime")]
    pub revision_creation_time: DateTime,

    pub creation_context: CreationContext,
    pub moderation_result: ModerationResult,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetUploadResponse {
    pub path: String,
    // TODO: cast to u64 please
    #[serde(rename = "assetId")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub name: String,

    pub state: String,
    pub asset_type: AssetTypeId,

    pub revision_id: String,
    #[serde(rename = "revisionCreateTime")]
    pub revision_creation_time: DateTime,

    pub creation_context: CreationContext,
    pub moderation_result: ModerationResult,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetUploadStatus {
    pub path: String,
    pub operation_id: String,
    #[serde(rename = "done")]
    pub complete: bool,
    pub response: Option<AssetUploadResponse>,
}

endpoint! {
    asset(id: u64) -> AssetInfo {
        GET "{URL}/assets/{id}";
    }

    status(operation_id: &str) -> AssetUploadStatus {
        GET "{URL}/operations/{operation_id}";
    }
}

// TODO: this api also takes in a patch request to update an exists asset "{URL}/assets/{id}"
pub async fn upload(
    client: &mut Client,
    bytes: &[u8],
    title: &str,
    description: &str,
    asset_type: AssetTypeId,
    creation_context: CreationContext,
) -> Result<AssetUploadStatus, Error> {
    let mut headers = client.requestor.default_headers.clone();
    headers.insert(header::ACCEPT, HeaderValue::from_str("*/*").unwrap());

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Request<'a> {
        #[serde(rename = "displayName")]
        title: &'a str,
        description: &'a str,
        asset_type: AssetTypeId,
        creation_context: CreationContext,
    }

    let request = serde_json::to_string(&Request {
        title,
        description,
        asset_type,
        creation_context,
    })
    .unwrap();

    let mime = match asset_type {
        AssetTypeId::Model => "model/x-rbxm",
        _ => infer::get(bytes)
            .map(|t| t.mime_type())
            .unwrap_or("application/octet-stream"),
    };

    let file_part = Part::bytes(bytes.to_owned())
        .file_name("instance")
        .mime_str(mime)?;

    let result = client
        .requestor
        .client
        .post(format!("{URL}/assets"))
        .headers(headers)
        .multipart(
            Form::new()
                .text("request", request)
                .part("fileContent", file_part),
        )
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<AssetUploadStatus>(response)
        .await
}
