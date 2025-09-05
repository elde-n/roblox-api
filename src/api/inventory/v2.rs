use serde::Deserialize;

use crate::{AssetTypeId, DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://inventory.roblox.com/v2";

#[derive(Clone, Debug, Deserialize)]
pub struct FromOwnerAssetOwner {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FromOwnerAssetInfo {
    pub id: u64,
    #[serde(rename = "serialNumber")]
    pub serial: u64,
    #[serde(rename = "collectibleItemInstanceId")]
    pub collectible_instance_id: Option<String>,
    pub owner: UserOwnedAssetOwner,
    pub created: DateTime,
    pub updated: DateTime,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserOwnedAssetOwner {
    #[serde(rename = "userId")]
    pub id: u64,
    #[serde(rename = "username")]
    pub name: String,
    // TODO: change this to an enum
    #[serde(rename = "buildersClubMembershipType")]
    pub premium_membership_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserOwnedAssetInfo {
    #[serde(rename = "assetId")]
    pub id: u64,
    #[serde(rename = "userAssetId")]
    pub instance_id: u64,
    #[serde(rename = "assetName")]
    pub name: String,
    pub owner: UserOwnedAssetOwner,
    #[serde(rename = "collectibleItemId")]
    pub collectible_id: Option<String>,
    #[serde(rename = "collectibleItemInstanceId")]
    pub collectible_instance_id: Option<String>,
    #[serde(rename = "serialNumber")]
    pub serial: Option<u64>,
    pub created: DateTime,
    pub updated: DateTime,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserOwnedAssets {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<UserOwnedAssetInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetOwners {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: String,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: String,
    #[serde(rename = "data")]
    pub assets: Vec<FromOwnerAssetInfo>,
}

pub async fn asset_owners(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<AssetOwners, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/assets/{id}/owners"))
        .query(&[
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<AssetOwners>(response).await
}

pub async fn user_owned_assets(
    client: &mut Client,
    user_id: u64,
    asset_type_id: AssetTypeId,
    paging: Paging<'_>,
) -> Result<UserOwnedAssets, Error> {
    let asset_type_id = asset_type_id as u8;

    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{user_id}/inventory/{asset_type_id}"))
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
        .parse_json::<UserOwnedAssets>(response)
        .await
}
