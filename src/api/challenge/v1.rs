use serde::Serialize;

use crate::{
    challenge::{Challenge, ChallengeMetadataRequest},
    endpoint,
};

pub const URL: &str = "https://apis.roblox.com/challenge/v1";

endpoint! {
    continue_challenge(challenge: &Challenge, verification_token: &str) -> () {
        POST "{URL}/continue";
        types {
            Request<'a> {
                id("challengeId"): &'a str,
                kind("challengeType"): &'a str,
                metadata("challengeMetadata"): &'a str,
            }
        }
        prelude {
            let metadata_json = serde_json::to_string(&ChallengeMetadataRequest {
                verification_token: verification_token.to_string(),
                challenge_id: challenge.metadata.server_challenge_id.clone(),
                action_type: challenge.metadata.action_type,
                remember_device: challenge.metadata.remember_device,
            })
            .unwrap();
        }
        body_serialize {
            &Request {
                id: &challenge.id,
                kind: &challenge.kind.to_string(),
                metadata: &metadata_json,
            }
        }
    }
}
