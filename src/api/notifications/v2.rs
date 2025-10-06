use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://notifications.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationUnreadCount {
    #[serde(rename = "unreadNotifications")]
    pub count: u64,
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClientEventsPayload {
    #[serde(rename = "sender_userid")]
    pub sender_user_id: Option<String>, // TODO: cast to u64
    pub place_id: Option<String>,      // TODO: cast to u64
    pub root_place_id: Option<String>, // TODO: cast to u64
    pub universe_id: Option<String>,   // TODO: cast to u64
    pub trigger: Option<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum VisualItemType {
    Button,
    TextBody,
    Thumbnail,
    MetaAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItemAction {
    pub path: String,
    pub next_state: String,     // Enum?
    pub action_type: String,    // Enum?
    pub fallback_state: String, // Enum?
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StyledText {
    pub text: String,
    pub styled_elements: Vec<StyledElement>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StyledElement {
    #[serde(rename = "styledElementType")]
    pub kind: String, // Enum?
    pub offset: i32,
    pub length: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItemTextBody {
    #[serde(rename = "visualItemType")]
    pub kind: VisualItemType,
    pub label: StyledText,
    pub title: Option<StyledText>,
    pub actions: Vec<VisualItemAction>,

    pub event_name: String,
    pub client_events_payload: ClientEventsPayload,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItemButton {
    #[serde(rename = "visualItemName")]
    pub name: String,

    #[serde(rename = "visualItemType")]
    pub kind: VisualItemType,
    pub label: StyledText,
    pub actions: Vec<VisualItemAction>,

    pub button_style: String, // Enum?
    pub event_name: String,
    pub client_events_payload: ClientEventsPayload,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItemThumbnail {
    pub id: String,
    pub id_type: String, // Enum?

    #[serde(rename = "visualItemType")]
    pub kind: VisualItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItemMetaAction {
    #[serde(rename = "visualItemName")]
    pub name: String,

    #[serde(rename = "visualItemType")]
    pub kind: VisualItemType,
    pub label: StyledText,
    pub actions: Vec<VisualItemAction>,

    pub action_icon: String, // Enum?
    pub event_name: String,
    pub client_events_payload: ClientEventsPayload,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VisualItems {
    pub button: Vec<VisualItemButton>,
    pub text_body: Vec<VisualItemTextBody>,
    pub thumbnail: Vec<VisualItemThumbnail>,
    pub meta_action: Vec<VisualItemMetaAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationContentStateDefault {
    pub layout_key: String, // Enum?
    pub visual_items: VisualItems,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationContent {
    pub notification_type: String, // Enum?

    pub min_version: String,
    pub time_before_refresh: String, // Cast to u64?
    pub client_events_payload: ClientEventsPayload,
    pub bundle_key: String,

    pub current_state: String, // Enum?
    pub states: HashMap<String, NotificationContentStateDefault>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub notification_source_type: String, // Enum?
    pub event_date: DateTime,
    pub timestamp: String,
    pub is_interacted: bool,

    pub metadata_collection: Vec<u64>,
    pub event_count: u64,

    pub content: NotificationContent,
}

pub async fn unread_count(client: &mut Client) -> Result<NotificationUnreadCount, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/stream-notifications/unread-count"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<NotificationUnreadCount>(response)
        .await
}

pub async fn recent(client: &mut Client, paging: Paging<'_>) -> Result<Vec<Notification>, Error> {
    let limit = paging.limit.unwrap_or(20).to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/stream-notifications/get-recent"))
        .query(&[("maxRows", limit), ("startIndex", cursor)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client
        .requestor
        .parse_json::<Vec<Notification>>(response)
        .await
}

pub async fn clear_unread(client: &mut Client) -> Result<String, Error> {
    let result = client
        .requestor
        .client
        .post(format!("{URL}/stream-notifications/clear-unread"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        status_message: String,
    }

    let response = client.requestor.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .status_message)
}

pub async fn dismiss(client: &mut Client, id: String) -> Result<String, Error> {
    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/stream-notifications/clear-unread/action/{id}/SpecialItemIgnoreAction"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        status_message: String,
    }

    let response = client.requestor.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .status_message)
}
