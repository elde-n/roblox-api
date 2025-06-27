use serde::Serialize;

use crate::{
    Error,
    challenge::{Challenge, ChallengeMetadataRequest},
    client::Client,
};

pub const URL: &str = "https://apis.roblox.com/challenge/v1";

pub async fn continue_challenge(
    client: &mut Client,
    challenge: &Challenge,
    verification_token: &str,
) -> Result<(), Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "challengeId")]
        id: &'a str,
        #[serde(rename = "challengeType")]
        kind: &'a str,
        #[serde(rename = "challengeMetadata")]
        metadata: &'a str,
    }

    let metadata_json = serde_json::to_string(&ChallengeMetadataRequest {
        verification_token: verification_token.to_string(),
        challenge_id: challenge.metadata.server_challenge_id.clone(),
        action_type: challenge.metadata.action_type,
        remember_device: challenge.metadata.remember_device,
    })
    .unwrap();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/continue"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            id: &challenge.id,
            kind: &challenge.kind.to_string(),
            metadata: &metadata_json,
        })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
