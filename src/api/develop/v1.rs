use serde::Deserialize;

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://develop.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PublishedAssetVersion {
    #[serde(rename = "Id")]
    pub id: u64,
    #[serde(rename = "assetId")]
    pub asset_id: u64,
    #[serde(rename = "assetVersionNumber")]
    pub asset_version: u64,
    #[serde(rename = "creatorTargetId")]
    pub creator_id: u64,
    #[serde(rename = "creatorType")]
    pub creator_type: String,
    #[serde(rename = "created")]
    pub created_at: DateTime,
    #[serde(rename = "isPublished")]
    pub is_published: bool,
    #[serde(rename = "isEqualToCurrentPublishedVersion")]
    pub is_current_published_version: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PublishedAssetVersions {
    #[serde(rename = "data")]
    pub assets: Vec<PublishedAssetVersion>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AssetCreator {
    #[serde(rename = "targetId")]
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "typeId")]
    pub type_id: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Asset {
    pub id: u64,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "typeId")]
    pub type_id: u32,

    #[serde(rename = "enableComments")]
    pub comments_enabled: bool,
    #[serde(rename = "isCopyingAllowed")]
    pub is_copying_allowed: bool,
    #[serde(rename = "isPublicDomainEnabled")]
    pub is_public_domain: bool,
    #[serde(rename = "isModerated")]
    pub is_moderated: bool,
    #[serde(rename = "isArchivable")]
    pub is_archivable: bool,
    #[serde(rename = "canHaveThumbnails")]
    pub thumbnails_allowed: bool,
    #[serde(rename = "isVersioningEnabled")]
    pub is_versioning_enabled: bool,

    #[serde(rename = "moderationStatus")]
    pub moderation_status: String,
    #[serde(rename = "reviewStatus")]
    pub review_status: String,

    #[serde(rename = "created")]
    pub creation_date: DateTime,
    #[serde(rename = "updated")]
    pub last_updated_date: DateTime,

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
    let result = client
        .requestor
        .client
        .get(format!("{URL}/assets/{id}/published-versions"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PublishedAssetVersions>(response)
        .await
}

pub async fn revert_asset_version(
    client: &mut Client,
    id: u64,
    asset_version: u64,
) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/assets/{id}/revert-version?assetVersionNumber={asset_version}"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
