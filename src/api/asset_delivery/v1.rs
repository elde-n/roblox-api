use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://assetdelivery.roblox.com/v1";
pub const SECONDARY_URL: &str = "https://apis.roblox.com/asset-delivery-api/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionContext {
    IgnoreUniverse,
}

pub async fn asset(client: &mut Client, id: u64) -> Result<Vec<u8>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/asset?id={id}"))
        .query(&[("id", id.to_string())])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;

    let bytes = response.bytes().await;
    match bytes {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(error) => Err(Error::ReqwestError(error)),
    }
}
