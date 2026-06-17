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

endpoint! {
    is_blocked(id: u64) -> bool {
        GET "{URL}/users/{id}/is-blocked";
    }

    batch_check_reciprocal_block(requester_id: u64, ids: &[u64]) -> Vec<UserBlockStatus> {
        POST "{URL}/users/batch-check-reciprocal-block";

        types {
            Request<'a> {
                requester_id("requesterUserId"): u64,
                ids("userIds"): &'a [u64],
            }
            Response {
                users: Vec<UserBlockStatus>,
            }
        }

        body_serialize {
            Request { requester_id, ids }
        }

        map |res: Response| res.users
    }
}
