use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{DateTime, Paging, endpoint};

pub const URL: &str = "https://badges.roblox.com/v1";

#[derive(Copy, Clone, Debug, PartialEq, Eq, Display, EnumString)]
pub enum BadgeSortBy {
    Rank,
    DateCreated,
}

// TODO: use CreatorType instead
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumString)]
pub enum BadgeCreatorType {
    User,
    Group,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumString)]
pub enum BadgeAwarderType {
    Place,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStatistics {
    #[serde(rename = "pastDayAwardedCount")]
    pub awarded_today: u32,
    #[serde(rename = "awardedCount")]
    pub awarded_total: u32,
    pub win_rate_percentage: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BadgeCreator {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: BadgeCreatorType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BadgeAwarder {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: BadgeAwarderType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BadgeUniverse {
    pub id: u64,
    pub name: String,
    pub root_place_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub description: String,

    pub display_name: String,
    pub display_description: String,

    pub enabled: bool,

    pub created: DateTime,
    pub updated: DateTime,

    pub icon_image_id: u64,
    pub display_icon_image_id: u64,

    pub statistics: BadgeStatistics,

    pub creator: Option<BadgeCreator>,
    pub awarder: Option<BadgeAwarder>,
    #[serde(rename = "awardingUniverse")]
    pub universe: Option<BadgeUniverse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BadgesResponse {
    #[serde(rename = "data")]
    pub badges: Vec<Badge>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

endpoint! {
    information(id: u64) -> Badge {
        GET "{URL}/badges/{id}";
    }

    universe_badges(id: u64, sort_by: Option<BadgeSortBy>, paging: Paging<'_>) -> BadgesResponse {
        GET "{URL}/universes/{id}/badges";
        paging_query { paging, limit = 10 }
        prelude {
            let sort_by_lower = sort_by.as_ref().map(|s| s.to_string().to_lowercase());
        }
        query {
            "sortBy" => sort_by_lower.as_deref().unwrap_or(""),
        }
    }

    user_badges(id: u64, paging: Paging<'_>) -> BadgesResponse {
        GET "{URL}/users/{id}/badges";
        paging_query { paging, limit = 10 }
    }

    remove(id: u64, user_id: u64) -> () {
        DELETE "{URL}/user/{user_id}/badges/{id}";
    }

    authenticated_remove(id: u64) -> () {
        DELETE "{URL}/user/badges/{id}";
    }
}
