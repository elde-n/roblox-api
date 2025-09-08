use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://accountinformation.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RobloxBadge {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub image_url: String,
}

pub async fn roblox_badges(client: &mut Client, id: u64) -> Result<Vec<RobloxBadge>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/roblox-badges"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<Vec<RobloxBadge>>(response)
        .await
}
