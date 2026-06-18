use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

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

endpoint! {
    information(id: u64) -> GroupInformation {
        GET "{URL}/groups/{id}";
    }

    /// Gets group membership information in the context of the authenticated user
    membership(id: u64, notification_preferences: bool) -> Membership {
        GET "{URL}/groups/{id}/membership";
        prelude {
            let notification_preferences = notification_preferences.to_string();
        }
        query {
            "includeNotificationPreferences" => &notification_preferences,
        }
    }

    /// Gets the Group's name change history
    name_history(id: u64) -> NameHistory {
        GET "{URL}/groups/{id}/name-history";
        types {
            NameHistoryItem {
                name: String,
                created: DateTime,
            }
            Response {
                items("data"): Vec<NameHistoryItem>,
                next_cursor("nextPageCursor"): Option<String>,
                previous_cursor("previousPageCursor"): Option<String>,
            }
        }
        map |r: Response| NameHistory {
            names: r.items.into_iter().map(|x| (x.name, x.created)).collect(),
            next_cursor: r.next_cursor,
            previous_cursor: r.previous_cursor,
        }
    }

    /// Gets groups that the authenticated user has requested to join
    pending_join_requests() -> Vec<GroupInformation> {
        GET "{URL}/user/groups/pending";
        types {
            Response {
                groups("data"): Vec<GroupInformation>,
            }
        }
        map |r: Response| r.groups
    }

    roles(id: u64) -> Vec<GroupRole> {
        GET "{URL}/groups/{id}/roles";
        types {
            Response {
                roles: Vec<GroupRole>,
            }
        }
        map |r: Response| r.roles
    }

    user_roles(id: u64) -> Vec<(GroupInformation, GroupRole)> {
        GET "{URL}/users/{id}/groups/roles";
        types {
            GroupAndRole {
                group: GroupInformation,
                role: GroupRole,
            }
            Response {
                items("data"): Vec<GroupAndRole>,
            }
        }
        map |r: Response| r.items.into_iter().map(|x| (x.group, x.role)).collect()
    }

    /// Gets the permissions for a group's roleset. The authorized user must either be the group owner or the roleset being requested, except for guest roles, which can be viewed by all (members and guests).
    roleset_permissions(id: u64, roleset_id: u64) -> RolePermissions {
        GET "{URL}/groups/{id}/roles/{roleset_id}/permissions";
    }

    /// Gets all permissions for each role
    role_permissions(id: u64) -> Vec<RolePermissions> {
        GET "{URL}/groups/{id}/roles/permissions";
        types {
            Response {
                items("data"): Vec<RolePermissions>,
            }
        }
        map |r: Response| r.items
    }

    users(id: u64, paging: Paging<'_>) -> GroupUsers {
        GET "{URL}/groups/{id}/users";
        paging_query { paging, limit = 10 }
        types {
            User {
                user: GroupUser,
                role: GroupRole,
            }
            Response {
                users("data"): Vec<User>,
                next_cursor("nextPageCursor"): Option<String>,
                previous_cursor("previousPageCursor"): Option<String>,
            }
        }
        map |r: Response| GroupUsers {
            users: r.users.into_iter().map(|u| (u.user, u.role)).collect(),
            next_cursor: r.next_cursor,
            previous_cursor: r.previous_cursor,
        }
    }

    /// Gets a list of group wall posts
    wall_posts(id: u64, paging: Paging<'_>) -> WallPosts {
        GET "{URL}/groups/{id}/wall/posts";
        paging_query { paging, limit = 10 }
    }

    join(id: u64) -> () {
        POST "{URL}/groups/{id}/users";
        types {
            Request<'a> {
                session_id: &'a str,
                redemption_token: &'a str,
            }
        }
        body_serialize { Request { session_id: "", redemption_token: "" } }
    }

    remove_join_request(id: u64, user_id: u64) -> () {
        DELETE "{URL}/groups/{id}/join-requests/users/{user_id}";
        types { Request {} }
        body_serialize { Request {} }
    }

    remove(id: u64, user_id: u64) -> () {
        DELETE "{URL}/groups/{id}/users/{user_id}";
        types { Request {} }
        body_serialize { Request {} }
    }
}
