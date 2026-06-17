use crate::{AssetTypeId, DateTime, Paging, endpoint};

pub const URL: &str = "https://apis.roblox.com/toolbox-service/v1";

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreationObject {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Creations {
    #[serde(rename = "totalResults")]
    pub results: u16,
    pub filtered_keyword: String,
    #[serde(rename = "data")]
    pub objects: Vec<CreationObject>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetailAsset {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub type_id: u32,
    pub duration: u32,
    #[serde(rename = "visibilityStatus")]
    pub visibility: u32,
    pub is_endorsed: bool,
    pub has_scripts: bool,
    pub is_asset_hash_approved: bool,
    #[serde(rename = "createdUtc")]
    pub created: DateTime,
    #[serde(rename = "updatedUtc")]
    pub updated: DateTime,
    #[serde(rename = "assetGenres")]
    pub genres: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ItemDetailCreator {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: u64,
    #[serde(rename = "isVerifiedCreator")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetailVotes {
    #[serde(rename = "upVotes")]
    pub likes: u32,
    #[serde(rename = "downVotes")]
    pub dislikes: u32,
    #[serde(rename = "voteCount")]
    pub votes: u32,
    #[serde(rename = "votePercent")]
    pub like_ratio: f32,
    pub show_votes: bool,
    pub can_vote: bool,
    pub has_voted: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FiatProductPriceQuantity {
    pub significand: u32,
    pub exponent: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FiatProductPrice {
    pub currency_code: String,
    pub quantity: FiatProductPriceQuantity,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FiatProduct {
    #[serde(rename = "purchasePrice")]
    pub price: FiatProductPrice,
    pub published: bool,
    pub purchasable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetail {
    pub asset: ItemDetailAsset,
    pub creator: ItemDetailCreator,
    pub votes: ItemDetailVotes,
    pub fiat_product: FiatProduct,
}

endpoint! {
    item_details(ids: &[u64]) -> Vec<ItemDetail> {
        GET "{URL}/items/details";
        types {
            Response {
                data: Vec<ItemDetail>,
            }
        }
        prelude {
            let ids = ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        }
        query { "assetIds" => &ids }
        map |r: Response| r.data
    }

    creations(id: u64, asset_type: AssetTypeId, paging: Paging<'_>) -> Creations {
        GET "{URL}/creations/user/{id}/{asset_type_id}";
        paging_query { paging, limit = 30 }
        prelude {
            let asset_type_id = asset_type as u8;
        }
    }
}
