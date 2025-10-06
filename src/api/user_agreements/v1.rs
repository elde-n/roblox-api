use serde::{Deserialize, Serialize};

use crate::{Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/user-agreements/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceResponse {
    #[serde(rename = "agreementId")]
    pub id: String,
    pub message: String,
    pub error_code: u16,
}

pub async fn acceptances(
    client: &mut Client,
    ids: &[&str],
) -> Result<Vec<AcceptanceResponse>, Error> {
    #[derive(Debug, Serialize)]
    struct Agreement<'a> {
        #[serde(rename = "agreementId")]
        id: &'a str,
    }

    #[derive(Debug, Serialize)]
    struct Request<'a> {
        acceptances: &'a [Agreement<'a>],
    }

    let acceptances = ids.iter().map(|x| Agreement { id: x }).collect::<Vec<_>>();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/acceptances"))
        .json(&Request {
            acceptances: &acceptances,
        })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        results: Vec<AcceptanceResponse>,
    }

    let response = client.requestor.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .results)
}
