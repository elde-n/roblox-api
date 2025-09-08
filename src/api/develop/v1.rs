use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://develop.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublishedAssetVersion {
    #[serde(rename = "Id")]
    pub id: u64,
    pub asset_id: u64,
    #[serde(rename = "assetVersionNumber")]
    pub asset_version: u64,
    #[serde(rename = "creatorTargetId")]
    pub creator_id: u64,
    pub creator_type: String,
    pub created: DateTime,
    pub is_published: bool,
    #[serde(rename = "isEqualToCurrentPublishedVersion")]
    pub is_current_published_version: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PublishedAssetVersions {
    #[serde(rename = "data")]
    pub assets: Vec<PublishedAssetVersion>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreator {
    #[serde(rename = "targetId")]
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: u64,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub type_id: u32,

    #[serde(rename = "enableComments")]
    pub comments_enabled: bool,
    pub is_copying_allowed: bool,
    #[serde(rename = "isPublicDomainEnabled")]
    pub is_public_domain: bool,
    pub is_moderated: bool,
    pub is_archivable: bool,
    #[serde(rename = "canHaveThumbnail")]
    pub thumbnails_allowed: bool,
    pub is_versioning_enabled: bool,

    pub moderation_status: Option<String>,
    pub review_status: String,

    pub created: DateTime,
    pub updated: DateTime,

    pub genres: Vec<String>,
    pub creator: AssetCreator,
}

pub async fn assets(client: &mut Client, ids: &[u64]) -> Result<Vec<Asset>, Error> {
    let ids = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let result = client
        .requestor
        .client
        .get(format!("{URL}/assets?assetIds={ids}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        assets: Vec<Asset>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .assets)
}

pub async fn published_asset_versions(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<PublishedAssetVersions, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/assets/{id}/published-versions"))
        .query(&[
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PublishedAssetVersions>(response)
        .await
}

pub async fn revert_asset_version(client: &mut Client, id: u64, version: u64) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/assets/{id}/revert-version?assetVersionNumber={version}"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
