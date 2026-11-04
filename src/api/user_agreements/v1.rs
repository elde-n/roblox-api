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

#[endpoint(POST, "{URL}/agreements/acceptances")]
async fn acceptances(agreements: &[&str]) -> AgreementAcceptances {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "agreements")]
        pub agreements: &'a [&'a str],
    }
    body! {
        Request { agreements }
    }
}
