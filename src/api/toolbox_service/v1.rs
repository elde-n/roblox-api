use serde::Deserialize;

use crate::{AssetTypeId, DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://apis.roblox.com/toolbox-service/v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct CreationObject {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Creations {
    #[serde(rename = "totalResults")]
    pub results: u16,
    #[serde(rename = "filteredKeyword")]
    pub filtered_keyword: String,
    #[serde(rename = "data")]
    pub objects: Vec<CreationObject>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ItemDetailAsset {
    pub id: u64,
    pub name: String,
    pub description: String,
    #[serde(rename = "typeId")]
    pub type_id: u32,
    pub duration: u32,
    #[serde(rename = "visibilityStatus")]
    pub visibility: u32,
    #[serde(rename = "isEndorsed")]
    pub is_endorsed: bool,
    #[serde(rename = "hasScripts")]
    pub has_scripts: bool,
    #[serde(rename = "isAssetHashApproved")]
    pub is_asset_hash_approved: bool,
    #[serde(rename = "createdUtc")]
    pub creation_date: DateTime,
    #[serde(rename = "updatedUtc")]
    pub last_updated_date: DateTime,
    //#[serde(rename = "assetSubTypes")]
    //pub sub_types: Vec<String?>,
    //#[serde(rename = "socialLinks")]
    //pub social_links: Vec<String?>,
    // pub model_technical_details: ModelTechnicalDetails?
    #[serde(rename = "assetGenres")]
    pub genres: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ItemDetailCreator {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: u64,
    #[serde(rename = "isVerifiedCreator")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ItemDetailVotes {
    #[serde(rename = "upVotes")]
    pub likes: u32,
    #[serde(rename = "downVotes")]
    pub dislikes: u32,
    #[serde(rename = "voteCount")]
    pub votes: u32,
    #[serde(rename = "votePercent")]
    pub like_ratio: f32,
    #[serde(rename = "showVotes")]
    pub show_votes: bool,
    #[serde(rename = "canVote")]
    pub can_vote: bool,
    #[serde(rename = "hasVoted")]
    pub has_voted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct FiatProductPriceQuantity {
    pub significand: u32,
    pub exponent: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct FiatProductPrice {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub quantity: FiatProductPriceQuantity,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct FiatProduct {
    #[serde(rename = "purchasePrice")]
    pub price: FiatProductPrice,
    pub published: bool,
    pub purchasable: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ItemDetail {
    pub asset: ItemDetailAsset,
    pub creator: ItemDetailCreator,
    pub votes: ItemDetailVotes,
    #[serde(rename = "fiatProduct")]
    pub fiat_product: FiatProduct,
}

pub async fn item_details(client: &mut Client, ids: &[u64]) -> Result<Vec<ItemDetail>, Error> {
    let ids = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let result = client
        .requestor
        .client
        .get(format!("{URL}/items/details?assetIds={ids}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct Response {
        #[serde(rename = "data")]
        objects: Vec<ItemDetail>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .objects)
}

pub async fn creations(
    client: &mut Client,
    id: u64,
    asset_type: AssetTypeId,
    paging: Paging<'_>,
) -> Result<Creations, Error> {
    let limit = paging.limit.unwrap_or(30);
    let cursor = match paging.cursor {
        Some(cursor) => format!("&cursor={cursor}"),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!(
            "{URL}/creations/user/{id}/{}?limit={limit}{cursor}",
            asset_type as u8
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Creations>(response).await
}
