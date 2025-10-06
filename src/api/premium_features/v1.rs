use crate::{Error, client::Client};

pub const URL: &str = "https://premiumfeatures.roblox.com/v1";

pub async fn is_premium(client: &mut Client, id: u64) -> Result<bool, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/validate-membership"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<bool>(response).await
}
