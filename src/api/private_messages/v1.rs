use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

pub const URL: &str = "https://privatemessages.roblox.com/v1";

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageTab {
    #[default]
    Inbox,
    Sent,
    Archive,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: u64,
    pub subject: String,
    pub body: String,

    pub sender: User,
    pub recipient: User,

    pub created: DateTime,
    pub updated: DateTime,

    pub is_read: bool,
    pub is_system_message: bool,
    pub is_report_abuse_displayed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Announcement {
    pub id: u64,
    pub subject: String,
    pub body: String,

    pub sender: User,

    pub created: DateTime,
    pub updated: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Announcements {
    pub collection: Vec<Announcement>,
    pub total_collection_size: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Messages {
    pub collection: Vec<Message>,
    pub total_collection_size: u64,
    pub total_pages: u64,
    #[serde(rename = "pageNumber")]
    pub current_page: u64,
}

impl std::fmt::Display for MessageTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MessageTab::Inbox => "inbox",
                MessageTab::Sent => "sent",
                MessageTab::Archive => "archive",
            }
        )
    }
}

#[endpoint(GET, "{URL}/messages/unread/count")]
async fn unread_count() -> u64 {
    #[derive(Serialize, Deserialize)]
    struct CountResponse {
        pub count: u64,
    }
    map! { |r: CountResponse| r.count }
}

/// The paging cursor is a page number
#[endpoint(GET, "{URL}/messages")]
async fn messages(tab: MessageTab, paging: Paging<'_>) -> Messages {
    let tab = tab.to_string();
    let limit = paging.limit.unwrap_or(100).to_string();
    let cursor = paging.cursor.map_or(String::new(), |c| c.to_string());

    query! {
        "messageTab" => &tab,
        "pageNumber" => &cursor,
        "pageSize" => &limit
    }
}

#[endpoint(GET, "{URL}/announcements")]
async fn announcements() -> Announcements {}

#[endpoint(POST, "{URL}/messages/archive")]
async fn archive(ids: &[u64]) -> Vec<u64> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "messageIds")]
        pub message_ids: &'a [u64],
    }
    #[derive(Serialize, Deserialize)]
    struct ActionResponse {
        #[serde(rename = "failedMessages")]
        pub failed: Vec<u64>,
    }
    body! { Request { message_ids: ids } }
    map! { |r: ActionResponse| r.failed }
}

#[endpoint(POST, "{URL}/messages/unarchive")]
async fn unarchive(ids: &[u64]) -> Vec<u64> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "messageIds")]
        pub message_ids: &'a [u64],
    }
    #[derive(Serialize, Deserialize)]
    struct ActionResponse {
        #[serde(rename = "failedMessages")]
        pub failed: Vec<u64>,
    }
    body! { Request { message_ids: ids } }
    map! { |r: ActionResponse| r.failed }
}

#[endpoint(POST, "{URL}/messages/mark-read")]
async fn mark_as_read(ids: &[u64]) -> Vec<u64> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "messageIds")]
        pub message_ids: &'a [u64],
    }
    #[derive(Serialize, Deserialize)]
    struct ActionResponse {
        #[serde(rename = "failedMessages")]
        pub failed: Vec<u64>,
    }
    body! { Request { message_ids: ids } }
    map! { |r: ActionResponse| r.failed }
}

#[endpoint(POST, "{URL}/messages/mark-unread")]
async fn mark_as_unread(ids: &[u64]) -> Vec<u64> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "messageIds")]
        pub message_ids: &'a [u64],
    }
    #[derive(Serialize, Deserialize)]
    struct ActionResponse {
        #[serde(rename = "failedMessages")]
        pub failed: Vec<u64>,
    }
    body! { Request { message_ids: ids } }
    map! { |r: ActionResponse| r.failed }
}
