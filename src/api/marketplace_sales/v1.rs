use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Currency, endpoint};

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

#[endpoint(POST, "{URL}/item/{asset_id}/purchase-item")]
async fn purchase(
    asset_id: &str,
    product_id: &str,
    price: u64,
    currency: Currency,
    purchaser: MarketEntity,
    seller: MarketEntity,
) -> PurchaseResponse {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "collectibleItemId")]
        pub asset_id: &'a str,
        #[serde(rename = "collectibleProductId")]
        pub product_id: &'a str,
        #[serde(rename = "expectedCurrency")]
        pub currency: u8,
        #[serde(rename = "expectedPrice")]
        pub price: u64,
        #[serde(rename = "expectedPurchaserId")]
        pub user_id: u64,
        #[serde(rename = "expectedPurchaserType")]
        pub purchaser_type: MarketEntityType,
        #[serde(rename = "expectedSellerId")]
        pub seller_id: u64,
        #[serde(rename = "expectedSellerType")]
        pub seller_type: MarketEntityType,
        #[serde(rename = "idempotencyKey")]
        pub idempotency_key: &'a str,
    }
    let idempotency_key = Uuid::new_v4().to_string();

    body! {
        Request {
            asset_id,
            product_id,
            currency: currency as u8,
            price,
            user_id: purchaser.id,
            purchaser_type: purchaser.kind,
            seller_id: seller.id,
            seller_type: seller.kind,
            idempotency_key: &idempotency_key,
        }
    }
}
