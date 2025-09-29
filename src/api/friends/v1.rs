use reqwest::Method;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{DateTime, Error, Paging, client::Client};

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

    pub source_universe_id: u64,
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

async fn generic_request<'a, R: Serialize, T: DeserializeOwned>(
    client: &mut Client,
    method: Method,
    path: &str,
    request: Option<&'a R>,
    query: Option<&'a [(&'a str, &'a str)]>,
) -> Result<T, Error> {
    let mut builder = client
        .requestor
        .client
        .request(method, format!("{URL}/{path}"))
        .headers(client.requestor.default_headers.clone());

    // Even though sending None works, it might get serialized as null in json, which is a waste of bytes
    if let Some(request) = request {
        builder = builder.json(&request);
    }

    if let Some(query) = query {
        builder = builder.query(&query);
    }

    let response = client.validate_response(builder.send().await).await?;
    client.requestor.parse_json::<T>(response).await
}

async fn generic_count(client: &mut Client, path: &str) -> Result<u16, Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        count: u16,
    }

    Ok(
        generic_request::<(), Response>(client, Method::GET, &format!("{path}/count"), None, None)
            .await?
            .count,
    )
}

pub async fn friend_requests_count(client: &mut Client) -> Result<u16, Error> {
    generic_count(client, "user/friend-requests").await
}

pub async fn new_friend_requests_count(client: &mut Client) -> Result<u16, Error> {
    generic_count(client, "my/new-friend-requests").await
}

pub async fn user_friends_count(client: &mut Client, id: u64) -> Result<u16, Error> {
    generic_count(client, &format!("users/{id}/friends")).await
}

pub async fn user_followings_count(client: &mut Client, id: u64) -> Result<u16, Error> {
    generic_count(client, &format!("users/{id}/followings")).await
}

pub async fn user_followers_count(client: &mut Client, id: u64) -> Result<u16, Error> {
    generic_count(client, &format!("users/{id}/followers")).await
}

pub async fn following_status(
    client: &mut Client,
    ids: &[u64],
) -> Result<Vec<FollowingStatus>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "targetUserIds")]
        user_ids: &'a [u64],
    }

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "followings")]
        statuses: Vec<FollowingStatus>,
    }

    Ok(generic_request::<Request, Response>(
        client,
        Method::POST,
        "user/following-exists",
        Some(&Request { user_ids: ids }),
        None,
    )
    .await?
    .statuses)
}

pub async fn friend_requests(
    client: &mut Client,
    paging: Paging<'_>,
) -> Result<FriendRequests, Error> {
    let limit = paging.limit.unwrap_or(18).to_string();
    let cursor = paging.cursor.unwrap_or("");

    generic_request::<(), FriendRequests>(
        client,
        Method::GET,
        "my/friends/requests",
        None,
        Some(&[("cursor", cursor), ("limit", &limit)]),
    )
    .await
}

pub async fn user_followers(client: &mut Client, id: u64) -> Result<Followers, Error> {
    generic_request::<(), Followers>(
        client,
        Method::GET,
        &format!("users/{id}/followers"),
        None,
        None,
    )
    .await
}

pub async fn user_followings(client: &mut Client, id: u64) -> Result<Followers, Error> {
    generic_request::<(), Followers>(
        client,
        Method::GET,
        &format!("users/{id}/followings"),
        None,
        None,
    )
    .await
}

pub async fn user_friends_online(
    client: &mut Client,
    id: u64,
) -> Result<Vec<FriendOnlineStatus>, Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        online: Vec<FriendOnlineStatus>,
    }

    Ok(generic_request::<(), Response>(
        client,
        Method::GET,
        &format!("users/{id}/friends/online"),
        None,
        None,
    )
    .await?
    .online)
}

pub async fn user_friends_find(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<FriendsFind, Error> {
    let limit = paging.limit.unwrap_or(18).to_string();
    let cursor = paging.cursor.unwrap_or("");

    generic_request::<(), FriendsFind>(
        client,
        Method::GET,
        &format!("users/{id}/friends/find"),
        None,
        Some(&[("cursor", cursor), ("limit", &limit), ("userSort", "1")]),
    )
    .await
}

pub async fn user_friends_search(
    client: &mut Client,
    id: u64,
    query: &str,
    paging: Paging<'_>,
) -> Result<FriendsFind, Error> {
    let limit = paging.limit.unwrap_or(36).to_string();
    let cursor = paging.cursor.unwrap_or("");

    generic_request::<(), FriendsFind>(
        client,
        Method::GET,
        &format!("users/{id}/friends/search"),
        None,
        Some(&[("cursor", &cursor), ("limit", &limit), ("query", query)]),
    )
    .await
}

pub async fn user_friend_statuses(
    client: &mut Client,
    id: u64,
    friends: &[u64],
) -> Result<Vec<FriendStatus>, Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        statuses: Vec<FriendStatus>,
    }

    let ids = friends
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok(generic_request::<(), Response>(
        client,
        Method::GET,
        &format!("users/{id}/friends/statuses"),
        None,
        Some(&[("userIds", &ids)]),
    )
    .await?
    .statuses)
}
