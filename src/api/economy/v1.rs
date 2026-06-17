use serde::{Deserialize, Serialize};

use crate::{Currency, endpoint};

pub const URL: &str = "https://economy.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PurchaseResponse {
    pub purchased: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct CurrencyResponse {
    robux: u64,
}

#[derive(Serialize)]
struct PurchaseRequest {
    #[serde(rename = "expectedCurrency")]
    currency: u8,
    #[serde(rename = "expectedPrice")]
    price: u64,
    #[serde(rename = "expectedSellerId")]
    seller_user_id: Option<u64>,
}

endpoint! {
    /// This api seems to only give an Internal Server Error, try using `marketplace_sales::v1::purchase` instead
    purchase(product_id: u64, price: u64, currency: Currency, seller_user_id: Option<u64>) -> PurchaseResponse {
        POST "{URL}/purchases/products/{product_id}";
        body_serialize { PurchaseRequest {
            currency: currency as u8,
            price,
            seller_user_id,
        }}
    }

    /// Returns how much `Currency::Robux` the authenticated user has
    currency() -> u64 {
        GET "{URL}/user/currency";
        map |r: CurrencyResponse| r.robux
    }

    /// Returns how much `Currency::Robux` the user has
    currency_from_user_id(id: u64) -> u64 {
        GET "{URL}/users/{id}/currency";
        map |r: CurrencyResponse| r.robux
    }

    /// Returns how much `Currency::Robux` the group has
    currency_from_group_id(id: u64) -> u64 {
        GET "{URL}/groups/{id}/currency";
        map |r: CurrencyResponse| r.robux
    }
}

// TODO:
// assets/{id}/resellers
// assets/{id}/resale-data
// groups/{id}/revenue/summary/{date?}
// groups/{id}/transactions - seems to be 404
