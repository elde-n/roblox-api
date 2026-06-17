use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

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

endpoint! {
    conversation_metadata() -> ConversationMetadata {
        GET "{URL}/get-conversation-metadata";
    }

    conversations_participant_metadata(ids: &[&str]) -> Vec<ConversationsParticipantMetadata> {
        POST "{URL}/get-conversations-participants-metadata";
        types {
            Request<'a> { ids("conversation_ids"): &'a [&'a str] }
            Response { metadata("conversation_participants_metadata"): HashMap<String, ParticipantsMetadata> }
            ParticipantsMetadata { participants_metadata: HashMap<String, ParticipantPending> }
            ParticipantPending { is_pending: bool }
        }
        body_serialize { Request { ids } }
        map |r: Response| {
            let mut metadata = Vec::new();
            for (conv_id, v) in &r.metadata {
                let mut participants = Vec::new();
                for (user_id, pending) in &v.participants_metadata {
                    participants.push(ParticipantMetadata {
                        id: user_id.parse().unwrap(),
                        is_pending: pending.is_pending,
                    });
                }
                metadata.push(ConversationsParticipantMetadata {
                    id: conv_id.to_owned(),
                    participants,
                });
            }
            metadata
        }
    }

    conversations(ids: &[&str]) -> Conversations {
        POST "{URL}/get-conversations";
        types {
            Request<'a> {
                ids: &'a [&'a str],
                include_messages: bool,
                include_user_data: bool,
                include_participants: bool,
            }
        }
        body_serialize {
            Request { ids, include_messages: true, include_user_data: true, include_participants: true }
        }
    }

    user_conversations(paging: Paging<'_>) -> Conversations {
        GET "{URL}/get-user-conversations";
        prelude {
            let limit = paging.limit.unwrap_or(20).to_string();
            let cursor_str = match paging.cursor {
                Some(c) => c.to_string(),
                None => String::new(),
            };
        }
        query {
            "cursor" => &cursor_str,
            "include_user_data" => "true",
            "pageSize" => &limit,
        }
    }

    conversation_messages(id: &str) -> ConversationMessages {
        GET "{URL}/get-conversation-messages";
        query { "conversation_id" => id }
    }

    /// Apparently you can only send 1 message at a time, but it's a vector in case roblox decides to change this behavior
    send_messages_in_conversation(id: &str, messages: &[&str]) -> ConversationMessages {
        POST "{URL}/send-messages";
        types {
            MessageToPost { content: String }
            Request<'a> { id("conversation_id"): &'a str, messages: &'a [MessageToPost] }
        }
        prelude {
            let msgs: Vec<MessageToPost> = messages.iter().map(|x| MessageToPost { content: x.to_string() }).collect();
        }
        body_serialize { Request { id, messages: &msgs } }
    }

    update_typing_status_in_conversation(id: &str) -> String {
        POST "{URL}/update-typing-status";
        types {
            Request<'a> { id("conversation_id"): &'a str }
            Response { status: String }
        }
        body_serialize { Request { id } }
        map |r: Response| r.status
    }

    add_users_to_conversation(id: &str, users: &[u64]) -> String {
        POST "{URL}/add-users";
        types {
            Request<'a> { id("conversation_id"): &'a str, users("user_ids"): &'a [u64] }
            Response { status: String }
        }
        body_serialize { Request { id, users } }
        map |r: Response| r.status
    }

    remove_users_from_conversation(id: &str, users: &[u64]) -> String {
        POST "{URL}/remove-users";
        types {
            Request<'a> { id("conversation_id"): &'a str, users("user_ids"): &'a [u64] }
            Response { status: String }
        }
        body_serialize { Request { id, users } }
        map |r: Response| r.status
    }

    create_conversations(conversations: &[ConversationCreateRequest]) -> Conversations {
        POST "{URL}/create-conversations";
        types {
            ConversationToCreate {
                name: String,
                kind("type"): String,
                users("participant_user_ids"): Vec<u64>,
            }
            Request<'a> { conversations: &'a [ConversationToCreate], include_user_data: bool }
        }
        prelude {
            let convs: Vec<ConversationToCreate> = conversations.iter().map(|x| ConversationToCreate {
                name: x.name.clone(),
                kind: "group".to_string(),
                users: x.users.clone(),
            }).collect();
        }
        body_serialize { Request { conversations: &convs, include_user_data: true } }
    }

    rename_conversations(ids: &[&str], names: &[&str]) -> Conversations {
        POST "{URL}/update-conversations";
        types {
            ConversationToUpdate { id: String, name: String }
            Request<'a> { conversations: &'a [ConversationToUpdate] }
        }
        prelude {
            let updates: Vec<ConversationToUpdate> = ids.iter()
                .zip(names.iter())
                .map(|(id, name)| ConversationToUpdate { id: id.to_string(), name: name.to_string() })
                .collect();
        }
        body_serialize { Request { conversations: &updates } }
    }

    mark_conversations_as_read(ids: &[&str]) -> Vec<ConversationMarkedStatus> {
        POST "{URL}/mark-conversations";
        types {
            Request<'a> { ids("conversation_ids"): &'a [&'a str] }
            Response { results: Vec<ConversationMarkedStatus> }
        }
        body_serialize { Request { ids } }
        map |r: Response| r.results
    }
}
