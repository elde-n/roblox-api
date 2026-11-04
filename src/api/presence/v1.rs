use serde::{Deserialize, Serialize};

use crate::endpoint;

pub const URL: &str = "https://presence.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(rename = "userId")]
    pub id: u64,
    #[serde(rename = "userPresenceType")]
    pub kind: u8,
    #[serde(rename = "lastLocation")]
    pub status: String,

    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub universe_id: Option<u64>,
    #[serde(rename = "gameId")]
    pub job_id: Option<String>,
}

#[endpoint(POST, "{URL}/presence/users")]
async fn presence(ids: &[u64]) -> Vec<UserPresence> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "userIds")]
        pub users: &'a [u64],
    }
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "userPresences")]
        pub presences: Vec<UserPresence>,
    }
    body! { Request { users: ids } }
    map! { |r: Response| r.presences }
}
