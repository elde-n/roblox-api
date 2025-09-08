use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://presence.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(rename = "userId")]
    pub id: u64,
    #[serde(rename = "userPresenceType")]
    pub kind: u8,
    #[serde(rename = "lastLocation")]
    pub status: String,

    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub universe_id: Option<u64>,
    #[serde(rename = "gameId")]
    pub job_id: Option<String>,
}

pub async fn presence(client: &mut Client, ids: &[u64]) -> Result<Vec<UserPresence>, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "userIds")]
        users: &'a [u64],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/presence/users"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request { users: ids })
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "userPresences")]
        presences: Vec<UserPresence>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .presences)
}
