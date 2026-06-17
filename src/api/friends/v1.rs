use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

pub const URL: &str = "https://friends.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FollowingStatus {
    #[serde(rename = "userId")]
    pub id: u64,
    pub is_following: bool,
    pub is_followed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FriendStatus {
    pub id: u64,
    pub status: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum FriendRequestSourceType {
    InGame,
    UserProfile,
    PlayerSearch,
    FriendRecommendations,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequester {
    #[serde(rename = "senderId")]
    pub id: u64,
    #[serde(rename = "senderNickname")]
    pub display_name: String,
    pub contact_name: Option<String>,

    pub source_universe_id: Option<u64>,
    pub origin_source_type: FriendRequestSourceType,
    pub sent_at: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
    pub id: u64,
    pub mutual_friends_list: Vec<String>,
    #[serde(rename = "friendRequest")]
    pub requester: FriendRequester,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequests {
    #[serde(rename = "data")]
    pub requests: Vec<FriendRequest>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    #[serde(rename = "data")]
    pub users: Vec<User>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct FriendsFind {
    #[serde(rename = "PageItems")]
    pub users: Vec<User>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    #[serde(rename = "UserPresenceType")]
    pub kind: String,
    #[serde(rename = "UserLocationType")]
    pub location_kind: String,

    #[serde(rename = "lastLocation")]
    pub status: String,
    pub last_online: DateTime,

    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub universe_id: Option<u64>,
    #[serde(rename = "gameInstanceId")]
    pub job_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FriendOnlineStatus {
    pub id: u64,
    #[serde(rename = "userPresence")]
    pub presence: UserPresence,
}

#[derive(Debug, Deserialize)]
struct CountResponse {
    count: u16,
}

endpoint! {
    friend_requests_count() -> u16 {
        GET "{URL}/user/friend-requests/count";
        map |r: CountResponse| r.count
    }

    new_friend_requests_count() -> u16 {
        GET "{URL}/my/new-friend-requests/count";
        map |r: CountResponse| r.count
    }

    user_friends_count(id: u64) -> u16 {
        GET "{URL}/users/{id}/friends/count";
        map |r: CountResponse| r.count
    }

    user_followings_count(id: u64) -> u16 {
        GET "{URL}/users/{id}/followings/count";
        map |r: CountResponse| r.count
    }

    user_followers_count(id: u64) -> u16 {
        GET "{URL}/users/{id}/followers/count";
        map |r: CountResponse| r.count
    }

    following_status(ids: &[u64]) -> Vec<FollowingStatus> {
        POST "{URL}/user/following-exists";
        types {
            Request<'a> {
                user_ids("targetUserIds"): &'a [u64],
            }
            Response {
                statuses("followings"): Vec<FollowingStatus>,
            }
        }
        body_serialize {
            &Request { user_ids: ids }
        }
        map |r: Response| r.statuses
    }

    friend_requests(paging: Paging<'_>) -> FriendRequests {
        GET "{URL}/my/friends/requests";
        paging_query { paging, limit = 18 }
    }

    user_followers(id: u64) -> Followers {
        GET "{URL}/users/{id}/followers";
    }

    user_followings(id: u64) -> Followers {
        GET "{URL}/users/{id}/followings";
    }

    user_friends_online(id: u64) -> Vec<FriendOnlineStatus> {
        GET "{URL}/users/{id}/friends/online";
        types {
            Response {
                online("data"): Vec<FriendOnlineStatus>,
            }
        }
        map |r: Response| r.online
    }

    user_friends_find(id: u64, paging: Paging<'_>) -> FriendsFind {
        GET "{URL}/users/{id}/friends/find";
        prelude {
            let limit = paging.limit.unwrap_or(18).to_string();
            let cursor = paging.cursor.unwrap_or("");
        }
        query {
            "cursor" => cursor,
            "limit" => &limit,
            "userSort" => "1",
        }
    }

    user_friends_search(id: u64, query: &str, paging: Paging<'_>) -> FriendsFind {
        GET "{URL}/users/{id}/friends/search";
        paging_query { paging, limit = 36 }
        query {
            "query" => query,
        }
    }

    user_friend_statuses(id: u64, friends: &[u64]) -> Vec<FriendStatus> {
        GET "{URL}/users/{id}/friends/statuses";
        types {
            Response {
                statuses("data"): Vec<FriendStatus>,
            }
        }
        prelude {
            let ids = friends
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        query {
            "userIds" => &ids,
        }
        map |r: Response| r.statuses
    }
}
