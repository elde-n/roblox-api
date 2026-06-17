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

endpoint! {
    unread_count() -> u64 {
        GET "{URL}/messages/unread/count";
        types { CountResponse { count: u64 } }
        map |r: CountResponse| r.count
    }

    /// The paging cursor is a page number
    messages(tab: MessageTab, paging: Paging<'_>) -> Messages {
        GET "{URL}/messages";
        prelude {
            let tab = tab.to_string();
            let limit = paging.limit.unwrap_or(100).to_string();
            let cursor = paging.cursor.map_or(String::new(), |c| c.to_string());
        }
        query {
            "messageTab" => &tab,
            "pageNumber" => &cursor,
            "pageSize" => &limit
        }
    }

    announcements() -> Announcements {
        GET "{URL}/announcements";
    }

    archive(ids: &[u64]) -> Vec<u64> {
        POST "{URL}/messages/archive";
        types {
            Request<'a> { message_ids("messageIds"): &'a [u64] }
            ActionResponse { failed("failedMessages"): Vec<u64> }
        }
        body_serialize { Request { message_ids: ids } }
        map |r: ActionResponse| r.failed
    }

    unarchive(ids: &[u64]) -> Vec<u64> {
        POST "{URL}/messages/unarchive";
        types {
            Request<'a> { message_ids("messageIds"): &'a [u64] }
            ActionResponse { failed("failedMessages"): Vec<u64> }
        }
        body_serialize { Request { message_ids: ids } }
        map |r: ActionResponse| r.failed
    }

    mark_as_read(ids: &[u64]) -> Vec<u64> {
        POST "{URL}/messages/mark-read";
        types {
            Request<'a> { message_ids("messageIds"): &'a [u64] }
            ActionResponse { failed("failedMessages"): Vec<u64> }
        }
        body_serialize { Request { message_ids: ids } }
        map |r: ActionResponse| r.failed
    }

    mark_as_unread(ids: &[u64]) -> Vec<u64> {
        POST "{URL}/messages/mark-unread";
        types {
            Request<'a> { message_ids("messageIds"): &'a [u64] }
            ActionResponse { failed("failedMessages"): Vec<u64> }
        }
        body_serialize { Request { message_ids: ids } }
        map |r: ActionResponse| r.failed
    }
}
