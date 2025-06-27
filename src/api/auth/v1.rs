use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, client::Client};

pub const URL: &str = "https://auth.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RecommendedUsernamesFromDisplayName {
    #[serde(rename = "didGenerateNewUsername")]
    pub new_name_generated: bool,
    #[serde(rename = "suggestedUsernames")]
    pub suggested_names: Vec<String>,
}

pub async fn recommended_usernames_from_display_name(
    client: &mut Client,
    display_name: &str,
    birthday: DateTime,
) -> Result<RecommendedUsernamesFromDisplayName, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "displayName")]
        display_name: &'a str,
        birthday: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/validators/recommendedUsernameFromDisplayName"
        ))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            display_name,
            birthday: birthday.to_string().as_str(),
        })
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<RecommendedUsernamesFromDisplayName>(response)
        .await
}
