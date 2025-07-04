use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::{Error, api::challenge, client::Client};

pub(crate) const CHALLENGE_ID_HEADER: &str = "rblx-challenge-id";
pub(crate) const CHALLENGE_TYPE_HEADER: &str = "rblx-challenge-type";
pub(crate) const CHALLENGE_METADATA_HEADER: &str = "rblx-challenge-metadata";

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ActionType {
    #[default]
    Unknown = 0,
    Login,
    RobuxSpend,
    ItemTrade,
    Resale,
    PasswordReset,
    RevertAccount,
    Generic,
    GenericWithRecoveryCodes,
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ChallengeType {
    #[default]
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "captcha")]
    Captcha,
    // This requires doing javascript challenges, however completing them is upto the user of this api
    #[serde(rename = "chef")]
    Chef,
    #[serde(rename = "twostepverification")]
    TwoStepVerification,
    // I'm not sure what this is, however I say it being referenced in some places
    #[serde(rename = "reauthentication")]
    Reauthentication,
    // I'm not sure what this is, however I say it being referenced in some places
    #[serde(rename = "security-questions")]
    SecurityQuestions,
}

impl From<&str> for ChallengeType {
    fn from(value: &str) -> Self {
        match value {
            "generic" => Self::Generic,
            "captcha" => Self::Captcha,
            "chef" => Self::Chef,
            "twostepverification" => Self::TwoStepVerification,
            "reauthentication" => Self::Reauthentication,
            "security-questions" => Self::SecurityQuestions,

            _ => Self::Generic,
        }
    }
}

impl std::fmt::Display for ChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChallengeMetadata {
    #[serde(rename = "userId")]
    pub(crate) user_id: String,
    #[serde(rename = "challengeId")]
    pub server_challenge_id: String,
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
    #[serde(rename = "rememberDevice")]
    pub(crate) remember_device: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChefChallengeMetadata {
    #[serde(rename = "userId")]
    pub(crate) user_id: String,
    #[serde(rename = "challengeId")]
    pub server_challenge_id: String,
    #[serde(rename = "expectedSymbols")]
    pub expected_symbols: Vec<String>,
    #[serde(rename = "scriptIdentifiers")]
    pub script_identifiers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Challenge {
    pub id: String,
    pub kind: ChallengeType,
    pub metadata: ChallengeMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct ChallengeMetadataRequest {
    #[serde(rename = "verificationToken")]
    pub(crate) verification_token: String,
    #[serde(rename = "challengeId")]
    pub(crate) challenge_id: String,
    #[serde(rename = "actionType")]
    pub(crate) action_type: ActionType,
    #[serde(rename = "rememberDevice")]
    pub(crate) remember_device: bool,
}

impl Client {
    // the name is misleading, there's no queue, also this function is kinda ugly to use,
    // perhaps it should be reworked
    pub async fn queue_challenge(
        &mut self,
        challenge: &Challenge,
        verification_token: &str,
    ) -> Result<(), Error> {
        // the challenge requires this api call, otherwise it fails
        challenge::v1::continue_challenge(self, &challenge, verification_token).await?;

        self.requestor.default_headers.insert(
            CHALLENGE_ID_HEADER,
            HeaderValue::from_str(&challenge.id).unwrap(),
        );

        self.requestor.default_headers.insert(
            CHALLENGE_TYPE_HEADER,
            HeaderValue::from_str(&challenge.kind.to_string()).unwrap(),
        );

        let metadata_b64 = BASE64_STANDARD.encode(
            serde_json::to_vec(
                &(ChallengeMetadataRequest {
                    verification_token: verification_token.to_string(),
                    challenge_id: challenge.metadata.server_challenge_id.clone(),
                    action_type: challenge.metadata.action_type,
                    remember_device: challenge.metadata.remember_device,
                }),
            )
            .unwrap(),
        );

        self.requestor.default_headers.insert(
            CHALLENGE_METADATA_HEADER,
            HeaderValue::from_str(&metadata_b64).unwrap(),
        );

        Ok(())
    }

    pub(crate) fn remove_challenge(&mut self) {
        self.requestor.default_headers.remove(CHALLENGE_ID_HEADER);
        self.requestor.default_headers.remove(CHALLENGE_TYPE_HEADER);
        self.requestor
            .default_headers
            .remove(CHALLENGE_METADATA_HEADER);
    }
}
