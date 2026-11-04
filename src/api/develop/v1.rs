use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

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

#[derive(Deserialize)]
struct AssetsResponse {
    #[serde(rename = "data")]
    assets: Vec<Asset>,
}

#[endpoint(GET, "{URL}/assets")]
async fn assets(ids: &[u64]) -> Vec<Asset> {
    let ids = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    query! {
        "assetIds" => &ids,
    }
    map! { |r: AssetsResponse| r.assets }
}

#[endpoint(GET, "{URL}/assets/{id}/published-versions")]
async fn published_asset_versions(id: u64, paging: Paging<'_>) -> PublishedAssetVersions {
    paging!(paging, limit = 10);
}

#[endpoint(POST, "{URL}/assets/{id}/revert-version?assetVersionNumber={version}")]
async fn revert_asset_version(id: u64, version: u64) -> () {
    void!();
}
