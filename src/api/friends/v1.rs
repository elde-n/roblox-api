use reqwest::Method;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{Error, client::Client};

pub const URL: &str = "https://friends.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FollowingStatus {
    #[serde(rename = "userId")]
    pub id: u64,
    pub is_following: bool,
    pub is_followed: bool,
}

async fn generic_request<'a, R: Serialize, T: DeserializeOwned>(
    client: &mut Client,
    method: Method,
    path: &str,
    request: Option<&'a R>,
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

    let response = client.validate_response(builder.send().await).await?;
    client.requestor.parse_json::<T>(response).await
}

async fn generic_count(client: &mut Client, path: &str) -> Result<u16, Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        count: u16,
    }

    Ok(
        generic_request::<(), Response>(client, Method::GET, &format!("{path}/count"), None)
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
    )
    .await?
    .statuses)
}

// TODO:
// add [GET] {URL}/my/friends/requests
// add [GET] {URL}/users/{id}/friends/online
// add [GET] {URL}/users/{id}/friends/statuses?userIds[]={id..}
// add [GET] {URL}/users/{id}/followers
// add [GET] {URL}/users/{id}/followings
// add [GET] {URL}/users/{id}/friends/find
// add [GET] {URL}/users/{id}/friends/search
