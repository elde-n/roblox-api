use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, endpoint};

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

#[endpoint(GET, "{URL}/get-conversation-metadata")]
async fn conversation_metadata() -> ConversationMetadata {}

#[endpoint(POST, "{URL}/get-conversations-participants-metadata")]
async fn conversations_participant_metadata(ids: &[&str]) -> Vec<ConversationsParticipantMetadata> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_ids")]
        pub ids: &'a [&'a str],
    }
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "conversation_participants_metadata")]
        pub metadata: HashMap<String, ParticipantsMetadata>,
    }
    #[derive(Serialize, Deserialize)]
    struct ParticipantsMetadata {
        pub participants_metadata: HashMap<String, ParticipantPending>,
    }
    #[derive(Serialize, Deserialize)]
    struct ParticipantPending {
        pub is_pending: bool,
    }
    body! { Request { ids } }
    try_map! { |r: Response| {
        let mut metadata = Vec::new();
        for (conv_id, v) in &r.metadata {
            let mut participants = Vec::new();
            for (user_id, pending) in &v.participants_metadata {
                participants.push(ParticipantMetadata {
                    id: user_id.parse().map_err(|_| Error::InvalidNumber)?,
                    is_pending: pending.is_pending,
                });
            }
            metadata.push(ConversationsParticipantMetadata {
                id: conv_id.to_owned(),
                participants,
            });
        }
        Ok(metadata)
    } }
}

#[endpoint(POST, "{URL}/get-conversations")]
async fn conversations(ids: &[&str]) -> Conversations {
    #[derive(Serialize)]
    struct Request<'a> {
        pub ids: &'a [&'a str],
        pub include_messages: bool,
        pub include_user_data: bool,
        pub include_participants: bool,
    }
    body! {
        Request { ids, include_messages: true, include_user_data: true, include_participants: true }
    }
}

#[endpoint(GET, "{URL}/get-user-conversations")]
async fn user_conversations(paging: Paging<'_>) -> Conversations {
    let limit = paging.limit.unwrap_or(20).to_string();
    let cursor_str = match paging.cursor {
        Some(c) => c.to_string(),
        None => String::new(),
    };

    query! {
        "cursor" => &cursor_str,
        "include_user_data" => "true",
        "pageSize" => &limit,
    }
}

#[endpoint(GET, "{URL}/get-conversation-messages")]
async fn conversation_messages(id: &str) -> ConversationMessages {
    query! { "conversation_id" => id }
}

/// Apparently you can only send 1 message at a time, but it's a vector in case roblox decides to change this behavior
#[endpoint(POST, "{URL}/send-messages")]
async fn send_messages_in_conversation(id: &str, messages: &[&str]) -> ConversationMessages {
    #[derive(Serialize, Deserialize)]
    struct MessageToPost {
        pub content: String,
    }
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        pub id: &'a str,
        pub messages: &'a [MessageToPost],
    }
    let msgs: Vec<MessageToPost> = messages
        .iter()
        .map(|x| MessageToPost {
            content: x.to_string(),
        })
        .collect();

    body! { Request { id, messages: &msgs } }
}

#[endpoint(POST, "{URL}/update-typing-status")]
async fn update_typing_status_in_conversation(id: &str) -> String {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        pub id: &'a str,
    }
    #[derive(Deserialize)]
    struct Response {
        pub status: String,
    }
    body! { Request { id } }
    map! { |r: Response| r.status }
}

#[endpoint(POST, "{URL}/add-users")]
async fn add_users_to_conversation(id: &str, users: &[u64]) -> String {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        pub id: &'a str,
        #[serde(rename = "user_ids")]
        pub users: &'a [u64],
    }
    #[derive(Deserialize)]
    struct Response {
        pub status: String,
    }
    body! { Request { id, users } }
    map! { |r: Response| r.status }
}

#[endpoint(POST, "{URL}/remove-users")]
async fn remove_users_from_conversation(id: &str, users: &[u64]) -> String {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_id")]
        pub id: &'a str,
        #[serde(rename = "user_ids")]
        pub users: &'a [u64],
    }
    #[derive(Deserialize)]
    struct Response {
        pub status: String,
    }
    body! { Request { id, users } }
    map! { |r: Response| r.status }
}

#[endpoint(POST, "{URL}/create-conversations")]
async fn create_conversations(conversations: &[ConversationCreateRequest]) -> Conversations {
    #[derive(Serialize, Deserialize)]
    struct ConversationToCreate {
        pub name: String,
        #[serde(rename = "type")]
        pub kind: String,
        #[serde(rename = "participant_user_ids")]
        pub users: Vec<u64>,
    }
    #[derive(Serialize)]
    struct Request<'a> {
        pub conversations: &'a [ConversationToCreate],
        pub include_user_data: bool,
    }
    let convs: Vec<ConversationToCreate> = conversations
        .iter()
        .map(|x| ConversationToCreate {
            name: x.name.clone(),
            kind: "group".to_string(),
            users: x.users.clone(),
        })
        .collect();

    body! { Request { conversations: &convs, include_user_data: true } }
}

#[endpoint(POST, "{URL}/update-conversations")]
async fn rename_conversations(ids: &[&str], names: &[&str]) -> Conversations {
    #[derive(Serialize, Deserialize)]
    struct ConversationToUpdate {
        pub id: String,
        pub name: String,
    }
    #[derive(Serialize)]
    struct Request<'a> {
        pub conversations: &'a [ConversationToUpdate],
    }
    let updates: Vec<ConversationToUpdate> = ids
        .iter()
        .zip(names.iter())
        .map(|(id, name)| ConversationToUpdate {
            id: id.to_string(),
            name: name.to_string(),
        })
        .collect();

    body! { Request { conversations: &updates } }
}

#[endpoint(POST, "{URL}/mark-conversations")]
async fn mark_conversations_as_read(ids: &[&str]) -> Vec<ConversationMarkedStatus> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "conversation_ids")]
        pub ids: &'a [&'a str],
    }
    #[derive(Deserialize)]
    struct Response {
        pub results: Vec<ConversationMarkedStatus>,
    }
    body! { Request { ids } }
    map! { |r: Response| r.results }
}
