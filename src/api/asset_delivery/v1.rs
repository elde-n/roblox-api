use serde::{Deserialize, Serialize};

use crate::endpoint;

pub const URL: &str = "https://assetdelivery.roblox.com/v1";
pub const SECONDARY_URL: &str = "https://apis.roblox.com/asset-delivery-api/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionContext {
    IgnoreUniverse,
}

endpoint! {
    asset(id: u64) -> Vec<u8> {
        GET "{URL}/asset";
        prelude {
            let id_str = id.to_string();
        }
        query {
            "id" => &id_str,
        }
        raw_bytes
    }
}
