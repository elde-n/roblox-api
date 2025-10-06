use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{DateTime, Error, Paging, client::Client};

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

async fn badges_generic<Response: DeserializeOwned>(
    client: &mut Client,
    path: &str,
    sort_by: Option<BadgeSortBy>,
    paging: Paging<'_>,
) -> Result<Response, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let sort_by = match sort_by {
        Some(sort_by) => sort_by.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/{path}/badges"))
        .query(&[
            ("sortBy", sort_by),
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<Response>(response).await
}

pub async fn information(client: &mut Client, id: u64) -> Result<Badge, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/badges/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<Badge>(response).await
}

pub async fn universe_badges(
    client: &mut Client,
    id: u64,
    sort_by: Option<BadgeSortBy>,
    paging: Paging<'_>,
) -> Result<BadgesResponse, Error> {
    badges_generic::<BadgesResponse>(client, &format!("universes/{id}"), sort_by, paging).await
}

pub async fn user_badges(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<BadgesResponse, Error> {
    badges_generic::<BadgesResponse>(client, &format!("users/{id}"), None, paging).await
}

pub async fn remove(client: &mut Client, id: u64, user_id: u64) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .delete(format!("{URL}/user/{user_id}/badges/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<()>(response).await
}

pub async fn authenticated_remove(client: &mut Client, id: u64) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .delete(format!("{URL}/user/badges/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<()>(response).await
}
