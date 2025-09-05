use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, Error, Paging, client::Client};

pub const URL: &str = "https://inventory.roblox.com/v1";

#[repr(u8)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemType {
    Asset = 0,
    Gamepass,
    Badge,
    Bundle,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetInfo {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    #[serde(rename = "instanceId")]
    pub instance_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserOwnsAssets {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<AssetInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CollectibleInfo {
    #[serde(rename = "assetId")]
    pub id: u64,
    #[serde(rename = "originalPrice")]
    pub original_price: u64,
    #[serde(rename = "recentAveragePrice")]
    pub recent_average_price: u64,
    #[serde(rename = "assetStock")]
    pub stock: u64,
    #[serde(rename = "userAssetId")]
    pub instance_id: u64,
    pub name: String,
    #[serde(rename = "buildersClubMembershipType")]
    pub premium_membership_type: String,
    #[serde(rename = "isOnHold")]
    pub on_hold: bool,
    #[serde(rename = "serialNumber")]
    pub serial: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserOwnedCollectibles {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<CollectibleInfo>,
}

pub async fn can_view_inventory(client: &mut Client, user_id: u64) -> Result<bool, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{user_id}/can-view-inventory"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        #[serde(rename = "canView")]
        can_view: bool,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .can_view)
}

pub async fn user_owns_assets(
    client: &mut Client,
    user_id: u64,
    id: u64,
    item_type: ItemType,
    paging: Paging<'_>,
) -> Result<UserOwnsAssets, Error> {
    let item_type = item_type as u8;

    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{user_id}/items/{item_type}/{id}"))
        .query(&[("cursor", cursor)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<UserOwnsAssets>(response)
        .await
}

pub async fn user_owned_collectibles(
    client: &mut Client,
    user_id: u64,
    asset_type_id: Option<AssetTypeId>,
    paging: Paging<'_>,
) -> Result<UserOwnedCollectibles, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let asset_type = match asset_type_id {
        Some(id) => {
            let id = id as u8;
            id.to_string()
        }
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{user_id}/assets/collectibles"))
        .query(&[
            ("assetType", asset_type),
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
        .parse_json::<UserOwnedCollectibles>(response)
        .await
}
