use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://groups.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: u64,
    pub name: String,
    pub rank: u8,

    /// How many users have the role
    pub member_count: Option<u64>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRole {
    pub user: GroupUser,
    pub role: GroupRole,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GroupShout {
    pub body: String,
    pub poster: GroupUser,
    pub created: DateTime,
    pub updated: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NameHistory {
    pub names: Vec<(String, DateTime)>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WallPost {
    pub id: u64,
    pub body: String,
    pub created: DateTime,
    pub updated: DateTime,
    pub poster: GroupUser,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WallPosts {
    #[serde(rename = "data")]
    pub posts: Vec<WallPost>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PostPermissions {
    pub view_wall: bool,
    pub post_to_wall: bool,
    pub delete_from_wall: bool,

    pub view_status: bool,
    pub post_to_status: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForumsPermissions {
    pub pin_posts: bool,
    pub lock_posts: bool,
    pub create_posts: bool,
    pub remove_posts: bool,

    pub create_comments: bool,
    pub remove_comments: bool,

    pub manage_categories: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentModerationPermissions {
    pub manage_keyword_block_list: bool,
    pub view_keyword_block_list: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MembershipPermissions {
    pub change_rank: bool,
    pub ban_members: bool,
    pub invite_members: bool,
    pub remove_members: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPermissions {
    pub manage_clan: bool,
    pub manage_relationships: bool,
    pub view_audit_logs: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EconomyPermissions {
    pub create_items: bool,
    pub manage_items: bool,
    pub advertise_group: bool,
    pub add_group_places: bool,
    pub spend_group_funds: bool,
    pub manage_group_games: bool,

    pub view_group_payouts: bool,
    pub view_analytics: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudPermissions {
    pub use_cloud_authentication: bool,
    pub administer_cloud_authentication: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    // TODO: all these fields are HashMap<String, bool>, but I can't figure a way to snakecase the hashmap keys, so.
    #[serde(rename = "groupPostsPermissions")]
    pub posts: PostPermissions,
    #[serde(rename = "groupForumsPermissions")]
    pub forums: ForumsPermissions,
    #[serde(rename = "groupContentModerationPermissions")]
    pub content_moderation: ContentModerationPermissions,
    #[serde(rename = "groupMembershipPermissions")]
    pub membership: MembershipPermissions,
    #[serde(rename = "groupManagementPermissions")]
    pub management: ManagementPermissions,
    #[serde(rename = "groupEconomyPermissions")]
    pub economy: EconomyPermissions,
    #[serde(rename = "groupOpenCloudPermissions")]
    pub open_cloud: OpenCloudPermissions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupInformation {
    pub id: u64,
    pub name: String,
    pub description: String,

    pub owner: Option<GroupUser>,
    pub shout: Option<GroupShout>,

    pub member_count: Option<u64>,
    #[serde(rename = "isBuildersClubOnly")]
    pub premium_only: bool,
    #[serde(rename = "publicEntryAllowed")]
    pub is_public: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NotificationPreference {
    #[serde(rename = "type")]
    pub name: String,
    pub description: String,

    pub kind: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    #[serde(rename = "groupId")]
    pub id: u64,

    /// GroupUser is the authenticated user
    pub user_role: UserRole,
    pub permissions: Permissions,

    pub is_primary: bool,
    pub is_pending_join: bool,

    pub are_enemies_allowed: bool,
    pub are_group_games_visible: bool,
    pub are_group_funds_visible: bool,

    pub can_configure: bool,

    pub notification_preferences: Option<Vec<NotificationPreference>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RolePermissions {
    #[serde(rename = "groupId")]
    pub id: u64,
    pub role: GroupRole,
    pub permissions: Permissions,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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

/// Gets group membership information in the context of the authenticated user
pub async fn membership(
    client: &mut Client,
    id: u64,
    notification_preferences: bool,
) -> Result<Membership, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/membership"))
        .query(&[("includeNotificationPreferences", notification_preferences)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Membership>(response).await
}

/// Gets the Group's name change history
pub async fn name_history(client: &mut Client, id: u64) -> Result<NameHistory, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/name-history"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct NameHistoryItem {
        name: String,
        created: DateTime,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        #[serde(rename = "data")]
        items: Vec<NameHistoryItem>,
        next_cursor: Option<String>,
        previous_cursor: Option<String>,
    }

    let response = client.validate_response(result).await?;
    let result = client.requestor.parse_json::<Response>(response).await?;

    let names: Vec<(String, DateTime)> = result
        .items
        .into_iter()
        .map(|x| (x.name, x.created))
        .collect();

    Ok(NameHistory {
        names,
        next_cursor: result.next_cursor,
        previous_cursor: result.previous_cursor,
    })
}

/// Gets groups that the authenticated user has requested to join
pub async fn pending_join_requests(client: &mut Client) -> Result<Vec<GroupInformation>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/user/groups/pending"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Clone, Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        groups: Vec<GroupInformation>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .groups)
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

/// Gets the permissions for a group's roleset. The authorized user must either be the group owner or the roleset being requested, except for guest roles, which can be viewed by all (members and guests).
pub async fn roleset_permissions(
    client: &mut Client,
    id: u64,
    roleset_id: u64,
) -> Result<RolePermissions, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/roles/{roleset_id}/permissions"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<RolePermissions>(response)
        .await
}

/// Gets all permissions for each role
pub async fn role_permissions(client: &mut Client, id: u64) -> Result<Vec<RolePermissions>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/roles/permissions"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        items: Vec<RolePermissions>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .items)
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

/// Gets a list of group wall posts
pub async fn wall_posts(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<WallPosts, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/groups/{id}/wall/posts"))
        .query(&[
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<WallPosts>(response).await
}

pub async fn join(client: &mut Client, id: u64) -> Result<(), Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Request<'a> {
        session_id: &'a str,
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
