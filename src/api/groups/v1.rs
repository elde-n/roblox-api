use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://groups.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupUser {
    #[serde(rename = "userId")]
    pub id: u64,
    #[serde(rename = "username")]
    pub name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupRole {
    pub id: u64,
    pub name: String,
    pub rank: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupShout {
    pub body: String,
    pub poster: GroupUser,
    pub created: DateTime,
    pub updated: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInformation {
    pub id: u64,
    pub name: String,
    pub description: String,

    pub owner: Option<GroupUser>,
    pub shout: Option<GroupShout>,

    pub member_count: u64,
    #[serde(rename = "isBuildersClubOnly")]
    pub premium_only: bool,
    #[serde(rename = "publicEntryAllowed")]
    pub is_public: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

pub struct GroupUsers {
    pub users: Vec<(GroupUser, GroupRole)>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

pub async fn information(client: &mut Client, id: u64) -> Result<GroupInformation, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<GroupInformation>(response)
        .await
}

pub async fn roles(client: &mut Client, id: u64) -> Result<Vec<GroupRole>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/roles"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        roles: Vec<GroupRole>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .roles)
}

pub async fn user_roles(
    client: &mut Client,
    id: u64,
) -> Result<Vec<(GroupInformation, GroupRole)>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}/groups/roles"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct GroupAndRole {
        group: GroupInformation,
        role: GroupRole,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        items: Vec<GroupAndRole>,
    }

    let response = client.validate_response(result).await?;
    let response = client.requestor.parse_json::<Response>(response).await?;

    let mut roles = Vec::new();
    for item in &response.items {
        roles.push((item.group.clone(), item.role.clone()));
    }

    Ok(roles)
}

pub async fn users(client: &mut Client, id: u64, paging: Paging<'_>) -> Result<GroupUsers, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/users"))
        .query(&[
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct User {
        user: GroupUser,
        role: GroupRole,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        users: Vec<User>,
        #[serde(rename = "nextPageCursor")]
        next_cursor: Option<String>,
        #[serde(rename = "previousPageCursor")]
        previous_cursor: Option<String>,
    }

    let response = client.validate_response(result).await?;
    let response = client.requestor.parse_json::<Response>(response).await?;

    let mut users = Vec::new();
    for user in response.users {
        users.push((user.user, user.role))
    }

    Ok(GroupUsers {
        users,
        next_cursor: response.next_cursor,
        previous_cursor: response.previous_cursor,
    })
}

pub async fn join(client: &mut Client, id: u64) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "sessionId")]
        session_id: &'a str,
        #[serde(rename = "redemptionToken")]
        redemption_token: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/groups/{id}/users"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            session_id: "",
            redemption_token: "",
        })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

pub async fn remove_join_request(client: &mut Client, id: u64, user_id: u64) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request {}

    let result = client
        .requestor
        .client
        .delete(format!("{URL}/groups/{id}/join-requests/users/{user_id}"))
        .json(&Request {})
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

pub async fn remove(client: &mut Client, id: u64, user_id: u64) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Request {}

    let result = client
        .requestor
        .client
        .delete(format!("{URL}/groups/{id}/users/{user_id}"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {})
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
