use serde::Serialize;

use crate::{
    challenge::{Challenge, ChallengeMetadataRequest},
    endpoint,
};

pub const URL: &str = "https://apis.roblox.com/challenge/v1";

#[endpoint(POST, "{URL}/continue")]
async fn continue_challenge(challenge: &Challenge, verification_token: &str) -> () {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "challengeId")]
        pub id: &'a str,
        #[serde(rename = "challengeType")]
        pub kind: &'a str,
        #[serde(rename = "challengeMetadata")]
        pub metadata: &'a str,
    }
    let metadata_json = serde_json::to_string(&ChallengeMetadataRequest {
        verification_token: verification_token.to_string(),
        challenge_id: challenge.metadata.server_challenge_id.clone(),
        action_type: challenge.metadata.action_type,
        remember_device: challenge.metadata.remember_device,
    })?;

    body! {
        &Request {
            id: &challenge.id,
            kind: &challenge.kind.to_string(),
            metadata: &metadata_json,
        }
    }
}
