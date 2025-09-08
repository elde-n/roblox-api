use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

pub const URL: &str = "https://economy.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ProductType {
    #[serde(rename = "Collectible Item")]
    Collectible,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CreatorType {
    User,
    Group,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Creator {
    pub id: u64,
    pub name: String,
    #[serde(rename = "CreatorType")]
    pub kind: CreatorType,
    #[serde(rename = "CreatorTargetId")]
    pub target_id: u64,
    #[serde(rename = "HasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SaleLocation {
    #[serde(rename = "SaleLocationType")]
    pub kind: u8,
    pub universe_ids: Vec<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CollectibleDetails {
    #[serde(rename = "CollectibleLowestResalePrice")]
    pub lowest_resale_price: Option<u64>,
    #[serde(rename = "CollectibleLowestAvailableResaleProductId")]
    pub lowest_available_product_id: Option<String>,
    #[serde(rename = "CollectibleLowestAvailableResaleItemInstanceId")]
    pub lowest_available_instance_id: Option<String>,
    #[serde(rename = "CollectibleQuantityLimitPerUser")]
    pub quantity_limit_per_user: Option<u64>,

    pub is_for_sale: bool,
    pub is_limited: bool,
    #[serde(rename = "TotalQuantity")]
    pub quantity: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DetailsResponse {
    #[serde(rename = "AssetId")]
    pub id: u64,
    pub target_id: u64,
    pub product_id: u64,
    pub product_type: ProductType,

    pub name: String,
    pub description: String,
    pub asset_type_id: u8,
    pub creator: Creator,

    #[serde(rename = "IconImageAssetId")]
    pub icon_image_asset_id: u64,
    pub created: DateTime,
    pub updated: DateTime,

    #[serde(rename = "PriceInRobux")]
    pub robux_price: u64,
    #[serde(rename = "PriceInTickets")]
    pub tickets_price: Option<u64>,
    pub sales: u64,
    pub remaining: u64,
    #[serde(rename = "IsForSale")]
    pub on_sale: bool,

    pub is_new: bool,
    pub is_public_domain: bool,
    pub is_limited: bool,
    pub is_limited_unique: bool,

    pub minimum_membership_level: u8,
    pub content_rating_type_id: u8,
    pub sale_availability_locations: Option<Vec<String>>, // im not sure
    pub sale_location: SaleLocation,

    #[serde(rename = "CollectibleItemId")]
    pub collectible_id: String,
    pub collectible_product_id: String,
    #[serde(rename = "CollectiblesItemDetails")]
    pub collectible_details: CollectibleDetails,
}

pub async fn details(client: &mut Client, id: u64) -> Result<DetailsResponse, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/assets/{id}/details"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<DetailsResponse>(response)
        .await
}
