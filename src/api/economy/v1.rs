use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://economy.roblox.com/v1";

#[repr(u8)]
#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub enum Currency {
    #[default]
    Robux = 1,
    Tickets,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PurchaseResponse {
    pub purchased: bool,
}

pub async fn purchase(
    client: &mut Client,
    product_id: u64,
    price: u64,
    currency: Currency,
    seller_user_id: Option<u64>,
) -> Result<PurchaseResponse, Error> {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename = "expectedCurrency")]
        currency: u8,
        #[serde(rename = "expectedPrice")]
        price: u64,
        //#[serde(rename = "expectedSellerId")]
        //seller_user_id: Option<u64>,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/purchases/products/{product_id}"))
        .json(&Request {
            currency: currency as u8,
            price,
            //seller_user_id,
        })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PurchaseResponse>(response)
        .await
}

// TODO:
// user/{id}/currency
// assets/{id}/resellers
// assets/{id}/resale-data
// groups/{id}/currency
// groups/{id}/revenue/summary/{date?}
// groups/{id}/transactions
//
