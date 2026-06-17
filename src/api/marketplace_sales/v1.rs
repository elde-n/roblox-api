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

endpoint! {
    purchase(asset_id: &str, product_id: &str, price: u64, currency: Currency, purchaser: MarketEntity, seller: MarketEntity) -> PurchaseResponse {
        POST "{URL}/item/{asset_id}/purchase-item";

        types {
            Request<'a> {
                asset_id("collectibleItemId"): &'a str,
                product_id("collectibleProductId"): &'a str,
                currency("expectedCurrency"): u8,
                price("expectedPrice"): u64,
                user_id("expectedPurchaserId"): u64,
                purchaser_type("expectedPurchaserType"): MarketEntityType,
                seller_id("expectedSellerId"): u64,
                seller_type("expectedSellerType"): MarketEntityType,
                idempotency_key("idempotencyKey"): &'a str,
            }
        }

        prelude {
            let idempotency_key = Uuid::new_v4().to_string();
        }

        body_serialize {
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
}
