use serde::{Deserialize, Serialize};

use crate::endpoint;

pub const URL: &str = "https://apis.roblox.com/user-blocking-api/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserBlockStatus {
    #[serde(rename = "userId")]
    pub id: u64,
    pub is_blocked: bool,
    pub is_blocking_viewer: bool,
}

#[endpoint(GET, "{URL}/users/{id}/is-blocked")]
async fn is_blocked(id: u64) -> bool {}

#[endpoint(POST, "{URL}/users/batch-check-reciprocal-block")]
async fn batch_check_reciprocal_block(requester_id: u64, ids: &[u64]) -> Vec<UserBlockStatus> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "requesterUserId")]
        pub requester_id: u64,
        #[serde(rename = "userIds")]
        pub ids: &'a [u64],
    }

    #[derive(Deserialize)]
    struct Response {
        pub users: Vec<UserBlockStatus>,
    }

    body! {
        Request { requester_id, ids }
    }
    map! { |res: Response| res.users }
}
