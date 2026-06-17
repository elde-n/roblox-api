use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::endpoint;

pub const URL: &str = "https://twostepverification.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum TwoStepMethodType {
    Authenticator,
    Email,
    #[serde(rename = "cross-device")]
    CrossDevice,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TwoStepMethods {
    pub is_user_verified: bool,
    pub primary: Option<TwoStepMethodType>,
    pub methods: Vec<TwoStepMethodType>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TwoStepVerify {
    pub is_two_step_verification_enabled: bool,
    pub ticket: Option<String>,
}

endpoint! {
    authenticator_verify(id: u64, code: &str, challenge_id: &str, method: &str) -> TwoStepVerify {
        POST "{URL}/users/{id}/channels/authenticator/verify";

        types {
            Request<'a> {
                code("code"): &'a str,
                challenge_id("challengeId"): &'a str,
                method("verificationMethod"): &'a str,
            }
        }

        body_serialize {
            Request { code, challenge_id, method }
        }
    }
}
