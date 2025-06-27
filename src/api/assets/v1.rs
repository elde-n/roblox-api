use std::path::Path;

use reqwest::{header::HeaderValue, multipart::Form};
use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

const URL: &str = "https://apis.roblox.com/assets/user-auth/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AssetType {
    Decal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Creator {
    #[serde(rename = "userId")]
    // TODO: cast to u64
    UserId(String), // can be sent as a u64, but it's returned as a string in some cases
    #[serde(rename = "groupId")]
    GroupId(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreationContext {
    pub creator: Creator,
    #[serde(rename = "expectedPrice")]
    pub expected_price: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModerationResult {
    #[serde(rename = "moderationState")]
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetUploadResponse {
    pub path: String,
    // TODO: cast to u64 please
    #[serde(rename = "assetId")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub title: String,
    #[serde(rename = "assetType")]
    pub asset_type: AssetType,
    pub state: String,
    #[serde(rename = "revisionId")]
    pub revision_id: String,
    #[serde(rename = "revisionCreateTime")]
    pub revision_creation_time: DateTime,
    #[serde(rename = "creationContext")]
    pub creation_context: CreationContext,
    #[serde(rename = "moderationResult")]
    pub moderation_result: ModerationResult,
}

#[derive(Debug, Deserialize)]
pub struct AssetUploadStatus {
    pub path: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "done")]
    pub complete: bool,
    pub response: Option<AssetUploadResponse>,
}

// this api also takes in a patch request to update an exists asset "{URL}/assets/{id}"
pub async fn upload(
    client: &mut Client,
    path: impl AsRef<Path>,
    title: &str,
    description: &str,
    asset_type: AssetType,
    creation_context: CreationContext,
) -> Result<AssetUploadStatus, Error> {
    let mut headers = client.requestor.default_headers.clone();
    headers.insert("Accept", HeaderValue::from_str("*/*").unwrap());

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct Request<'a> {
        #[serde(rename = "displayName")]
        title: &'a str,
        description: &'a str,
        #[serde(rename = "assetType")]
        asset_type: AssetType,
        #[serde(rename = "creationContext")]
        creation_context: CreationContext,
    }

    let request = serde_json::to_string(&Request {
        title,
        description,
        asset_type,
        creation_context,
    })
    .unwrap();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/assets"))
        .headers(headers)
        .multipart(
            Form::new()
                .text("request", request)
                .file("fileContent", &path)
                .await
                .unwrap(),
        )
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<AssetUploadStatus>(response)
        .await
}

pub async fn status(client: &mut Client, operation_id: &str) -> Result<AssetUploadStatus, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/operations/{operation_id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<AssetUploadStatus>(response)
        .await
}
