use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, DateTime, Paging, endpoint};

pub const URL: &str = "https://inventory.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FromOwnerAssetOwner {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOwnedAssetOwner {
    #[serde(rename = "userId")]
    pub id: u64,
    #[serde(rename = "username")]
    pub name: String,
    // TODO: change this to an enum
    #[serde(rename = "buildersClubMembershipType")]
    pub premium_membership_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetOwners {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: String,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: String,
    #[serde(rename = "data")]
    pub assets: Vec<FromOwnerAssetInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOwnedAssets {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
    #[serde(rename = "data")]
    pub assets: Vec<UserOwnedAssetInfo>,
}

endpoint! {
    asset_owners(id: u64, paging: Paging<'_>) -> AssetOwners {
        GET "{URL}/assets/{id}/owners";
        paging_query { paging, limit = 10 }
    }

    user_owned_assets(user_id: u64, asset_type_id: AssetTypeId, paging: Paging<'_>) -> UserOwnedAssets {
        GET "{URL}/users/{user_id}/inventory/{__aid}";
        paging_query { paging, limit = 10 }
        prelude {
            let __aid = asset_type_id as u8;
        }
    }
}
