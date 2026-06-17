use serde::{Deserialize, Serialize};

use crate::endpoint;

pub const URL: &str = "https://agreements.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Agreement {
    pub document_type: String,
    pub is_accepted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub accepted_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgreementAcceptances {
    pub acceptances: Vec<Agreement>,
}

endpoint! {
    acceptances(agreements: &[&str]) -> AgreementAcceptances {
        POST "{URL}/agreements/acceptances";

        types {
            Request<'a> {
                agreements("agreements"): &'a [&'a str],
            }
        }

        body_serialize {
            Request { agreements }
        }
    }
}
