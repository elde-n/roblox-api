use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://apis.roblox.com/game-passes/v1";

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GamepassCreator {
    #[serde(rename = "creatorId")]
    pub id: u64,
    pub name: String,
    #[serde(rename = "creatorType")]
    pub kind: CreatorType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Gamepass {
    #[serde(rename = "gamePassId")]
    pub id: u64,
    pub name: String,
    pub description: String,

    #[serde(rename = "iconAssetId")]
    pub icon_image_id: Option<u64>,

    pub price: Option<u64>,
    #[serde(rename = "isForSale")]
    pub on_sale: bool,
    pub creator: GamepassCreator,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PriceInformation {
    #[serde(rename = "defaultPriceInRobux")]
    pub price_in_robux: u64,
    pub enabled_features: Vec<String>, // Such as RegionalPricing
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GamepassDetails {
    #[serde(rename = "gamePassId")]
    pub id: u64,
    pub name: String,
    pub description: String,

    pub place_id: u64,
    #[serde(rename = "iconAssetId")]
    pub icon_image_id: u64,

    #[serde(rename = "createdTimestamp")]
    pub created: DateTime,
    #[serde(rename = "updatedTimestamp")]
    pub updated: DateTime,

    #[serde(rename = "isForSale")]
    pub on_sale: bool,
    pub price_information: Option<PriceInformation>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GamepassProductInformation {
    #[serde(rename = "TargetId")]
    pub id: u64,
    pub asset_id: u64,
    pub product_id: u64,

    pub name: String,
    pub description: String,

    pub product_type: String,
    pub asset_type_id: u8, // seems to be 0?

    pub creator: Creator,

    #[serde(rename = "IconImageAssetId")]
    pub icon_image_id: u64,

    pub created: DateTime,
    pub updated: DateTime,

    #[serde(rename = "PriceInRobux")]
    pub robux_price: Option<u64>,
    #[serde(rename = "PriceInTickets")]
    pub tickets_price: Option<u64>,

    pub sales: u64,
    pub remaining: Option<u64>,

    #[serde(rename = "IsForSale")]
    pub on_sale: bool,

    pub is_new: bool,
    pub is_public_domain: bool,
    pub is_limited: bool,
    pub is_limited_unique: bool,

    pub minimum_membership_level: u8,
}

pub async fn details(client: &mut Client, id: u64) -> Result<GamepassDetails, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/game-passes/{id}/details"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<GamepassDetails>(response)
        .await
}

pub async fn product_information(
    client: &mut Client,
    id: u64,
) -> Result<GamepassProductInformation, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/game-passes/{id}/product-info"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<GamepassProductInformation>(response)
        .await
}

/// The cursor is the gamepass_id you want to start from
pub async fn user_gamepasses(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<Vec<Gamepass>, Error> {
    let limit = paging.limit.unwrap_or(100).to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/game-passes"))
        .query(&[("count", limit), ("exclusiveStartId", cursor)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "gamePasses")]
        gamepasses: Vec<Gamepass>,
    }

    let response = client.requestor.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .gamepasses)
}
