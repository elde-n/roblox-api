use serde::{Deserialize, Serialize};

use crate::{Currency, Error, client::Client};

pub const URL: &str = "https://economy.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PurchaseResponse {
    pub purchased: bool,
}

/// This api seems to only give an Internal Server Error, try using `marketplace_sales::v1::purchase` instead
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
        #[serde(rename = "expectedSellerId")]
        seller_user_id: Option<u64>,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/purchases/products/{product_id}"))
        .json(&Request {
            currency: currency as u8,
            price,
            seller_user_id,
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

/// Returns how much `Currency::Robux` the authenticated user has
pub async fn currency(client: &mut Client) -> Result<u64, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/user/currency"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        robux: u64,
    }

    let response = client.validate_response(result).await?;
    let response = client.requestor.parse_json::<Response>(response).await?;

    Ok(response.robux)
}

/// Returns how much `Currency::Robux` the user has
pub async fn currency_from_user_id(client: &mut Client, id: u64) -> Result<u64, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/currency"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        robux: u64,
    }

    let response = client.validate_response(result).await?;
    let response = client.requestor.parse_json::<Response>(response).await?;

    Ok(response.robux)
}

// TODO:
// assets/{id}/resellers
// assets/{id}/resale-data
// groups/{id}/currency
// groups/{id}/revenue/summary/{date?}
// groups/{id}/transactions - seems to be 404
