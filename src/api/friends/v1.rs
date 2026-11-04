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

#[endpoint(GET, "{URL}/user/friend-requests/count")]
async fn friend_requests_count() -> u16 {
    map! { |r: CountResponse| r.count }
}

#[endpoint(GET, "{URL}/my/new-friend-requests/count")]
async fn new_friend_requests_count() -> u16 {
    map! { |r: CountResponse| r.count }
}

#[endpoint(GET, "{URL}/users/{id}/friends/count")]
async fn user_friends_count(id: u64) -> u16 {
    map! { |r: CountResponse| r.count }
}

#[endpoint(GET, "{URL}/users/{id}/followings/count")]
async fn user_followings_count(id: u64) -> u16 {
    map! { |r: CountResponse| r.count }
}

#[endpoint(GET, "{URL}/users/{id}/followers/count")]
async fn user_followers_count(id: u64) -> u16 {
    map! { |r: CountResponse| r.count }
}

#[endpoint(POST, "{URL}/user/following-exists")]
async fn following_status(ids: &[u64]) -> Vec<FollowingStatus> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "targetUserIds")]
        pub user_ids: &'a [u64],
    }
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "followings")]
        pub statuses: Vec<FollowingStatus>,
    }
    body! {
        &Request { user_ids: ids }
    }
    map! { |r: Response| r.statuses }
}

#[endpoint(GET, "{URL}/my/friends/requests")]
async fn friend_requests(paging: Paging<'_>) -> FriendRequests {
    paging!(paging, limit = 18);
}

#[endpoint(GET, "{URL}/users/{id}/followers")]
async fn user_followers(id: u64) -> Followers {}

#[endpoint(GET, "{URL}/users/{id}/followings")]
async fn user_followings(id: u64) -> Followers {}

#[endpoint(GET, "{URL}/users/{id}/friends/online")]
async fn user_friends_online(id: u64) -> Vec<FriendOnlineStatus> {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        pub online: Vec<FriendOnlineStatus>,
    }
    map! { |r: Response| r.online }
}

#[endpoint(GET, "{URL}/users/{id}/friends/find")]
async fn user_friends_find(id: u64, paging: Paging<'_>) -> FriendsFind {
    let limit = paging.limit.unwrap_or(18).to_string();
    let cursor = paging.cursor.unwrap_or("");

    query! {
        "cursor" => cursor,
        "limit" => &limit,
        "userSort" => "1",
    }
}

#[endpoint(GET, "{URL}/users/{id}/friends/search")]
async fn user_friends_search(id: u64, query: &str, paging: Paging<'_>) -> FriendsFind {
    paging!(paging, limit = 36);
    query! {
        "query" => query,
    }
}

#[endpoint(GET, "{URL}/users/{id}/friends/statuses")]
async fn user_friend_statuses(id: u64, friends: &[u64]) -> Vec<FriendStatus> {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        pub statuses: Vec<FriendStatus>,
    }
    let ids = friends
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    query! {
        "userIds" => &ids,
    }
    map! { |r: Response| r.statuses }
}
