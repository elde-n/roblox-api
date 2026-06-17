use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, Paging, endpoint};

pub const URL: &str = "https://inventory.roblox.com/v1";

#[repr(u8)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemType {
    Asset = 0,
    Gamepass,
    Badge,
    Bundle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub instance_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOwnsAssets {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<AssetInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectibleInfo {
    #[serde(rename = "assetId")]
    pub id: u64,
    pub original_price: u64,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOwnedCollectibles {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<CollectibleInfo>,
}

endpoint! {
    can_view_inventory(user_id: u64) -> bool {
        GET "{URL}/users/{user_id}/can-view-inventory";

        types {
            Response {
                can_view("canView"): bool,
            }
        }

        map |r: Response| r.can_view
    }

    user_owns_assets(user_id: u64, id: u64, item_type: ItemType, paging: Paging<'_>) -> UserOwnsAssets {
        GET "{URL}/users/{user_id}/items/{item_type_as_u8}/{id}";

        prelude {
            let item_type_as_u8 = item_type as u8;
            let cursor = match paging.cursor {
                Some(cursor) => cursor.to_string(),
                None => String::new(),
            };
        }

        query {
            "cursor" => &cursor,
        }
    }

    user_owned_collectibles(user_id: u64, asset_type_id: Option<AssetTypeId>, paging: Paging<'_>) -> UserOwnedCollectibles {
        GET "{URL}/users/{user_id}/assets/collectibles";

        paging_query { paging, limit = 10 }

        prelude {
            let asset_type = match asset_type_id {
                Some(id) => (id as u8).to_string(),
                None => String::new(),
            };
        }

        query {
            "assetType" => &asset_type,
        }
    }
}
