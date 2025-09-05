use serde::{Deserialize, Serialize};

use crate::{Error, challenge::ActionType, client::Client};

pub const URL: &str = "https://twostepverification.roblox.com/v1";

// TODO: I don't know what `user_id` is for, as this api only seems to be used for the client only,
// there's also currently no way to require id from Client, perhaps we should authenticate
// on from_cookie method, and store the ClientDetails in the Client
pub async fn authenticator_verify(
    client: &mut Client,
    user_id: u64,
    code: &str,
    action_type: ActionType,
    server_challenge_id: &str,
) -> Result<String, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "actionType")]
        action_type: &'a str,
        #[serde(rename = "challengeId")]
        challenge_id: &'a str,
        code: &'a str,
    }

    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "verificationToken")]
        verification_token: String,
    }

    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/users/{user_id}/challenges/authenticator/verify"
        ))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            action_type: &action_type.to_string(),
            challenge_id: server_challenge_id,
            code,
        })
        .send()
        .await;

    let response = client.validate_response(result).await?;
    let result = client.requestor.parse_json::<Response>(response).await?;

    Ok(result.verification_token)
}
