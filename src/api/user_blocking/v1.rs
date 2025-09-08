use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/user-blocking-api/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserBlockStatus {
    #[serde(rename = "userId")]
    pub id: u64,
    pub is_blocked: bool,
    pub is_blocking_viewer: bool,
}

pub async fn is_blocked(client: &mut Client, id: u64) -> Result<bool, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/is-blocked"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<bool>(response).await
}

pub async fn batch_check_reciprocal_block(
    client: &mut Client,
    requester_id: u64,
    ids: &[u64],
) -> Result<Vec<UserBlockStatus>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "requesterUserId")]
        requester_id: u64,
        #[serde(rename = "userIds")]
        ids: &'a [u64],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/users/batch-check-reciprocal-block"))
        .json(&Request { requester_id, ids })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        users: Vec<UserBlockStatus>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .users)
}
