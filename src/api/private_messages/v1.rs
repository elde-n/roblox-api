use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

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

async fn generic_message_action(
    client: &mut Client,
    path: &str,
    ids: &[u64],
) -> Result<Vec<u64>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "messageIds")]
        ids: &'a [u64],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/messages/{path}"))
        .json(&Request { ids })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "failedMessages")]
        failed: Vec<u64>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .failed)
}

pub async fn unread_count(client: &mut Client) -> Result<u64, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/messages/unread/count"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        count: u64,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .count)
}

/// The paging cursor is a page number
pub async fn messages(
    client: &mut Client,
    tab: MessageTab,
    paging: Paging<'_>,
) -> Result<Messages, Error> {
    let limit = paging.limit.unwrap_or(100).to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/messages"))
        .query(&[
            ("messageTab", tab.to_string()),
            ("pageNumber", cursor),
            ("pageSize", limit),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Messages>(response).await
}

pub async fn announcements(client: &mut Client) -> Result<Announcements, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/announcements"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Announcements>(response).await
}

pub async fn archive(client: &mut Client, ids: &[u64]) -> Result<Vec<u64>, Error> {
    generic_message_action(client, "archive", ids).await
}

pub async fn unarchive(client: &mut Client, ids: &[u64]) -> Result<Vec<u64>, Error> {
    generic_message_action(client, "unarchive", ids).await
}

pub async fn mark_as_read(client: &mut Client, ids: &[u64]) -> Result<Vec<u64>, Error> {
    generic_message_action(client, "mark-read", ids).await
}

pub async fn mark_as_unread(client: &mut Client, ids: &[u64]) -> Result<Vec<u64>, Error> {
    generic_message_action(client, "mark-unread", ids).await
}
