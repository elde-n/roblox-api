use serde::{Deserialize, Serialize};

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

async fn generic_count(client: &mut Client, path: &str) -> Result<u16, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/{path}/count"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        count: u16,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .count)
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

    let result = client
        .requestor
        .client
        .post(format!("{URL}/user/following-exists"))
        .json(&Request { user_ids: ids })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "followings")]
        statuses: Vec<FollowingStatus>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
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
