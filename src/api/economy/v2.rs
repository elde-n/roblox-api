use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

pub const URL: &str = "https://economy.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ProductType {
    #[serde(rename = "Collectible Item")]
    Collectible,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum CreatorType {
    User,
    Group,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Creator {
    #[serde(rename = "Id")]
    pub id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CreatorType")]
    pub kind: CreatorType,
    #[serde(rename = "CreatorTargetId")]
    pub target_id: u64,
    #[serde(rename = "HasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct SaleLocation {
    #[serde(rename = "SaleLocationType")]
    pub kind: u8,
    #[serde(rename = "UniverseIds")]
    pub universe_ids: Vec<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CollectibleDetails {
    #[serde(rename = "CollectibleLowestResalePrice")]
    pub lowest_resale_price: Option<u64>,
    #[serde(rename = "CollectibleLowestAvailableResaleProductId")]
    pub lowest_available_product_id: Option<String>,
    #[serde(rename = "CollectibleLowestAvailableResaleItemInstanceId")]
    pub lowest_available_instance_id: Option<String>,
    #[serde(rename = "CollectibleQuantityLimitPerUser")]
    pub quantity_limit_per_user: Option<u64>,

    #[serde(rename = "IsForSale")]
    pub is_for_sale: bool,
    #[serde(rename = "IsLimited")]
    pub is_limited: bool,
    #[serde(rename = "TotalQuantity")]
    pub quantity: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DetailsResponse {
    #[serde(rename = "AssetId")]
    pub id: u64,
    #[serde(rename = "TargetId")]
    pub target_id: u64,
    #[serde(rename = "ProductId")]
    pub product_id: u64,
    #[serde(rename = "ProductType")]
    pub product_type: ProductType,

    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "AssetTypeId")]
    pub asset_type_id: u8,
    #[serde(rename = "Creator")]
    pub creator: Creator,

    #[serde(rename = "IconImageAssetId")]
    pub icon_image_asset_id: u64,
    #[serde(rename = "Created")]
    pub created: DateTime,
    #[serde(rename = "Updated")]
    pub updated: DateTime,

    #[serde(rename = "PriceInRobux")]
    pub robux_price: u64,
    #[serde(rename = "PriceInTickets")]
    pub tickets_price: Option<u64>,
    #[serde(rename = "Sales")]
    pub sales: u64,
    #[serde(rename = "Remaining")]
    pub remaining: u64,
    #[serde(rename = "IsForSale")]
    pub on_sale: bool,

    #[serde(rename = "IsNew")]
    pub is_new: bool,
    #[serde(rename = "IsPublicDomain")]
    pub is_public_domain: bool,
    #[serde(rename = "IsLimited")]
    pub is_limited: bool,
    #[serde(rename = "IsLimitedUnique")]
    pub is_limited_unique: bool,

    #[serde(rename = "MinimumMembershipLevel")]
    pub minimum_membership_level: u8,
    #[serde(rename = "ContentRatingTypeId")]
    pub content_rating_type_id: u8,
    #[serde(rename = "SaleAvailabilityLocations")]
    pub sale_availability_locations: Option<Vec<String>>, // im not sure
    #[serde(rename = "SaleLocation")]
    pub sale_location: SaleLocation,
    #[serde(rename = "CollectibleItemId")]
    pub collectible_id: String,
    #[serde(rename = "CollectibleProductId")]
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
