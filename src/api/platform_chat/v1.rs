use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://apis.roblox.com/platform-chat-api/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationType {
    OneToOne,
    Group,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationSource {
    Channels,
    Friends,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationUser {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    // what in the fuck is this, seems to be just display_name.unwrap_or(name) so just display_name, cause display_name defaults to name if unset
    pub combined_name: String,
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: String, // maybe enum

    #[serde(rename = "sender_user_id")]
    pub sender_id: u64,
    pub replies_to: Option<()>, // to who?

    #[serde(rename = "created_at")]
    pub created: DateTime,

    pub moderation_type: String, // maybe enum

    pub is_deleted: bool,
    pub is_badgeable: bool,
    pub is_previewable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ConversationType,
    pub source: String, // maybe enum?

    #[serde(rename = "created_by")]
    pub creator_id: Option<u64>,
    #[serde(rename = "participant_user_ids")]
    pub participants: Vec<u64>,
    #[serde(rename = "user_data")]
    // I tried with serde_with to make this a Vec<V>, although I failed
    // TODO: refactor to Vec<ConversationUser>
    pub users: HashMap<String, ConversationUser>,

    pub messages: Vec<Message>,
    pub preview_message: Option<Message>,

    pub sort_index: u64,
    pub unread_message_count: u64,

    #[serde(rename = "created_at")]
    pub created: DateTime,
    #[serde(rename = "updated_at")]
    pub updated: DateTime,

    pub status: Option<String>,          // maybe enum
    pub moderation_type: Option<String>, // maybe enum

    pub user_pending_status: Option<String>, // maybe enum
    pub participant_pending_status: Option<String>, // maybe enum
    pub osa_acknowledgement_status: String,  // maybe enum

    pub is_default_name: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Conversations {
    pub conversations: Vec<Conversation>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationMessages {
    pub messages: Vec<Message>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationMetadata {
    pub global_unread_count: u64,
    pub global_unread_message_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParticipantMetadata {
    pub id: u64,
    pub is_pending: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationsParticipantMetadata {
    pub id: String,
    pub participants: Vec<ParticipantMetadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationMarkedStatus {
    #[serde(rename = "conversation_id")]
    pub id: String,
    pub status: String, // maybe enum?
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConversationCreateRequest {
    pub name: String,
    pub users: Vec<u64>,
}

pub async fn conversation_metadata(client: &mut Client) -> Result<ConversationMetadata, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/get-conversation-metadata"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<ConversationMetadata>(response)
        .await
}

pub async fn conversations_participant_metadata(
    client: &mut Client,
    ids: &[&str],
) -> Result<Vec<ConversationsParticipantMetadata>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_ids")]
        ids: &'a [&'a str],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/get-conversations-participants-metadata"))
        .json(&Request { ids })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct ParticipantPending {
        is_pending: bool,
    }

    #[derive(Debug, Deserialize)]
    struct ParticipantsMetadata {
        participants_metadata: HashMap<String, ParticipantPending>,
    }

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "conversation_participants_metadata")]
        metadata: HashMap<String, ParticipantsMetadata>,
    }

    let response = client.validate_response(result).await?;
    let response = client.requestor.parse_json::<Response>(response).await?;

    let mut metadata = Vec::new();
    for (k, v) in &response.metadata {
        let mut participants = Vec::new();
        for (k, v) in &v.participants_metadata {
            participants.push(ParticipantMetadata {
                id: k.parse().unwrap(),
                is_pending: v.is_pending,
            });
        }

        metadata.push(ConversationsParticipantMetadata {
            id: k.to_owned(),
            participants,
        })
    }

    Ok(metadata)
}

pub async fn conversations(client: &mut Client, ids: &[&str]) -> Result<Conversations, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        ids: &'a [&'a str],
        include_messages: bool,
        include_user_data: bool,
        include_participants: bool,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/get-conversations"))
        .json(&Request {
            ids,
            include_messages: true,
            include_user_data: true,
            include_participants: true,
        })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Conversations>(response).await
}

pub async fn user_conversations(
    client: &mut Client,
    paging: Paging<'_>,
) -> Result<Conversations, Error> {
    let limit = paging.limit.unwrap_or(20).to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/get-user-conversations"))
        .query(&[
            ("cursor", cursor),
            ("include_user_data", true.to_string()),
            ("pageSize", limit),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Conversations>(response).await
}

pub async fn conversation_messages(
    client: &mut Client,
    id: &str,
) -> Result<ConversationMessages, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/get-conversation-messages"))
        .query(&[("conversation_id", id)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<ConversationMessages>(response)
        .await
}

/// Apparently you can only send 1 message at a time, but it's a vector in case roblox decides to change this behavior
pub async fn send_messages_in_conversation(
    client: &mut Client,
    id: &str,
    messages: &[&str],
) -> Result<ConversationMessages, Error> {
    #[derive(Debug, Serialize)]
    struct MessageToPost<'a> {
        content: &'a str,
    }

    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        id: &'a str,
        messages: &'a [MessageToPost<'a>],
    }

    let messages = &messages
        .iter()
        .map(|x| MessageToPost { content: x })
        .collect::<Vec<_>>();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/send-messages"))
        .json(&Request { id, messages })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<ConversationMessages>(response)
        .await
}

pub async fn update_typing_status_in_conversation(
    client: &mut Client,
    id: &str,
) -> Result<String, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        id: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/update-typing-status"))
        .json(&Request { id })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        status: String,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .status)
}

pub async fn add_users_to_conversation(
    client: &mut Client,
    id: &str,
    users: &[u64],
) -> Result<String, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        id: &'a str,
        #[serde(rename = "user_ids")]
        users: &'a [u64],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/add-users"))
        .json(&Request { id, users })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        status: String,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .status)
}

pub async fn remove_users_from_conversation(
    client: &mut Client,
    id: &str,
    users: &[u64],
) -> Result<String, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        id: &'a str,
        #[serde(rename = "user_ids")]
        users: &'a [u64],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/remove-users"))
        .json(&Request { id, users })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        status: String,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .status)
}

pub async fn create_conversations(
    client: &mut Client,
    conversations: &[ConversationCreateRequest],
) -> Result<Conversations, Error> {
    #[derive(Debug, Serialize)]
    struct ConversationToCreate<'a> {
        name: &'a str,
        #[serde(rename = "type")]
        kind: &'a str,
        #[serde(rename = "participant_user_ids")]
        users: &'a [u64],
    }

    #[derive(Debug, Serialize)]
    struct Request<'a> {
        conversations: &'a [ConversationToCreate<'a>],
        include_user_data: bool,
    }

    let conversations = &conversations
        .iter()
        .map(|x| ConversationToCreate {
            name: &x.name,
            kind: "group",
            users: &x.users,
        })
        .collect::<Vec<_>>();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/create-conversations"))
        .json(&Request {
            conversations,
            include_user_data: true,
        })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Conversations>(response).await
}

pub async fn rename_conversations(
    client: &mut Client,
    ids: &[&str],
    names: &[&str],
) -> Result<Conversations, Error> {
    #[derive(Debug, Serialize)]
    struct ConversationToUpdate<'a> {
        id: &'a str,
        name: &'a str,
    }

    #[derive(Debug, Serialize)]
    struct Request<'a> {
        conversations: &'a [ConversationToUpdate<'a>],
    }

    let conversations = &ids
        .iter()
        .zip(names.into_iter())
        .collect::<Vec<_>>()
        .iter()
        .map(|(id, name)| ConversationToUpdate { id, name })
        .collect::<Vec<_>>();

    let result = client
        .requestor
        .client
        .post(format!("{URL}/update-conversations"))
        .json(&Request { conversations })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<Conversations>(response).await
}

pub async fn mark_conversations_as_read(
    client: &mut Client,
    ids: &[&str],
) -> Result<Vec<ConversationMarkedStatus>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_ids")]
        ids: &'a [&'a str],
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/mark-conversations"))
        .json(&Request { ids })
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        results: Vec<ConversationMarkedStatus>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .results)
}
