use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Currency, Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/marketplace-sales/v1";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum MarketEntityType {
    #[default]
    User,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MarketEntity {
    pub id: u64,
    pub kind: MarketEntityType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseResponse {
    pub pending: bool,
    pub purchased: bool,
    pub purchase_result: String,
    #[serde(rename = "errorMessage")]
    pub error: Option<String>,
}

pub async fn purchase(
    client: &mut Client,
    asset_id: &str,
    product_id: &str,
    price: u64,
    currency: Currency,
    purchaser: MarketEntity,
    seller: MarketEntity,
) -> Result<PurchaseResponse, Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Request<'a> {
        #[serde(rename = "collectibleItemId")]
        asset_id: &'a str,
        #[serde(rename = "collectibleProductId")]
        product_id: &'a str,
        #[serde(rename = "expectedCurrency")]
        currency: u8,
        #[serde(rename = "expectedPrice")]
        price: u64,
        #[serde(rename = "expectedPurchaserId")]
        user_id: u64, // ... why
        #[serde(rename = "expectedPurchaserType")]
        purchaser_type: MarketEntityType,
        #[serde(rename = "expectedSellerId")]
        seller_id: u64,
        #[serde(rename = "expectedSellerType")]
        seller_type: MarketEntityType,
        idempotency_key: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/item/{asset_id}/purchase-item"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            asset_id,
            product_id,
            currency: currency as u8,
            price,
            user_id: purchaser.id,
            purchaser_type: purchaser.kind,
            seller_id: seller.id,
            seller_type: seller.kind,
            idempotency_key: &Uuid::new_v4().to_string(),
        })
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PurchaseResponse>(response)
        .await
}
