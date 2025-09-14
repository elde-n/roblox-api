use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::{CloseCode, Message, RequestBuilderExt, WebSocket};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{Error, client::Client};

pub const URL: &str = "wss://realtime-signalr.roblox.com/userhub";
const TERMINATOR: &'static str = "\x1E";

pub struct Socket(WebSocket);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct SocketMessage {
    #[serde(rename = "type")]
    kind: SocketMessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<SocketMessageTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageResponse {
    Ping,
    SubscriptionStatus(SubscriptionStatus),
    CommunicationChannels(CommunicationChannels),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageRequest {
    Ping,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
pub enum SocketMessageType {
    Invocation = 1,
    Ping = 6,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SocketMessageTarget {
    SubscriptionStatus,
    CommunicationChannels,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PresenceType {
    PresenceChanged,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CommunicationChannelsType {
    MessageCreated,
    ParticipantTyping,
    ChannelMetadataUpdated,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ChannelType {
    PlatformChatGroup,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct PresenceBulkNotifications {
    pub user_id: u64,
    #[serde(rename = "Type")]
    pub kind: PresenceType,
    pub sequence_number: u64,
    pub realtime_message_identifier: u64,
    pub should_send_to_event_stream: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunicationChannelsActor {
    pub id: u64,
    #[serde(rename = "Type")]
    pub kind: String, // User
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct CommunicationChannelsNotificationTags {
    #[serde(rename = "type")]
    pub kind: String, // type: "group"
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunicationChannels {
    pub is_typing: Option<bool>,
    #[serde(rename = "Type")]
    pub kind: CommunicationChannelsType,
    pub actor: CommunicationChannelsActor,

    pub channel_id: String,
    pub channel_type: ChannelType,
    pub channel_vertical: String,

    pub notification_tags: CommunicationChannelsNotificationTags,

    pub sequence_number: u64,
    pub realtime_message_identifier: u64,

    pub is_live_channel: bool,
    pub should_send_to_event_stream: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionStatusNamespaceSequenceNumbers {
    pub chat_notifications: Option<u64>,
    pub voice_notifications: Option<u64>,
    pub game_close_notifications: Option<u64>,
    pub friendship_notifications: Option<u64>,
    pub display_name_notifications: Option<u64>,
    pub user_profile_notifications: Option<u64>,
    pub presence_bulk_notifications: Option<u64>,
    pub game_favorite_notifications: Option<u64>,
    pub authentication_notifications: Option<u64>,
    pub cloud_edit_chat_notifications: Option<u64>,
    pub avatar_asset_ownership_notifications: Option<u64>,
    pub avatar_outfit_ownership_notifications: Option<u64>,

    #[serde(rename = "toast-in-experience-notifications")]
    pub toast_in_experience_notifications: Option<u64>,
    #[serde(rename = "toast-in-app-and-experience-notifications")]
    pub toast_in_app_and_experience_notifications: Option<u64>,

    pub call_notification: Option<u64>,
    pub message_notification: Option<u64>,
    pub user_tag_change_notification: Option<u64>,
    pub user_theme_type_change_notification: Option<u64>,

    pub revenue_reports: Option<u64>,
    pub notification_stream: Option<u64>,
    pub communication_channels: Option<u64>,
    pub activity_history_event: Option<u64>,
    pub eligibility_status_changed: Option<u64>,
    pub chat_moderation_type_eligibility: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionStatus {
    pub connection_id: String,
    pub sequence_number: u64,
    pub milliseconds_before_handling_reconnect: u64,
    pub namespace_sequence_numbers: SubscriptionStatusNamespaceSequenceNumbers,
}

#[derive(Deserialize, Serialize)]
struct HandshakeRequest<'a> {
    protocol: &'a str,
    version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct HandshakeResponse {
    error: Option<String>,
}

async fn handshake(mut socket: Socket) -> Result<Socket, Error> {
    let _ = &mut socket
        .send_text(
            serde_json::to_string(&HandshakeRequest {
                protocol: "json",
                version: 1,
            })
            .unwrap()
            .as_str(),
        )
        .await?;

    let response = if let Message::Text(string) = socket.0.next().await.ok_or(Error::BadJson)?? {
        let messages: Vec<&str> = string.split_terminator(TERMINATOR).collect();
        let first = messages.first().unwrap();

        Some(serde_json::from_str::<HandshakeResponse>(first)?)
    } else {
        None
    }
    .ok_or(Error::BadJson)?;

    match response.error {
        None => Ok(socket),
        Some(_) => Err(Error::BadJson),
    }
}

pub async fn connect(client: &mut Client) -> Result<Socket, Error> {
    let socket = Socket(
        client
            .requestor
            .client
            .get(URL)
            .headers(client.requestor.default_headers.clone())
            .upgrade()
            .send()
            .await?
            .into_websocket()
            .await?,
    );

    handshake(socket).await
}

impl Socket {
    /// Reads the stream until a message is received
    pub async fn read(&mut self) -> Result<Vec<MessageResponse>, Error> {
        while let Some(message) = self.0.next().await {
            match message? {
                Message::Text(string) => {
                    let messages: Vec<&str> = string.split_terminator(TERMINATOR).collect();

                    let mut responses = Vec::new();
                    for m in &messages {
                        dbg!("Message: {}", m);

                        let message = serde_json::from_str::<SocketMessage>(m)?;
                        match message.kind {
                            SocketMessageType::Invocation => {
                                match message.target {
                                    Some(SocketMessageTarget::SubscriptionStatus) => {
                                        // arg.0 is event_type?
                                        let json = serde_json::from_str::<SubscriptionStatus>(
                                            &message.arguments.unwrap().get(1).unwrap().as_str(),
                                        )?;

                                        responses.push(MessageResponse::SubscriptionStatus(json));
                                    }
                                    Some(SocketMessageTarget::CommunicationChannels) => {
                                        let json = serde_json::from_str::<CommunicationChannels>(
                                            &message.arguments.unwrap().get(1).unwrap().as_str(),
                                        )?;

                                        responses
                                            .push(MessageResponse::CommunicationChannels(json));
                                    }

                                    None => {}
                                };
                            }

                            SocketMessageType::Ping => {
                                responses.push(MessageResponse::Ping);
                            }
                        }
                    }

                    return Ok(responses);
                }

                _ => {}
            }
        }

        Ok(Vec::new())
    }

    pub async fn send(&mut self, message: MessageRequest) -> Result<(), Error> {
        let json = match &message {
            MessageRequest::Ping => SocketMessage {
                kind: SocketMessageType::Ping,
                target: None,
                arguments: None,
            },
        };

        self.send_text(serde_json::to_string(&json)?.as_str()).await
    }

    pub async fn send_text(&mut self, message: &str) -> Result<(), Error> {
        Ok(self
            .0
            .send(Message::Text(format!("{}{TERMINATOR}", message)))
            .await?)
    }

    pub async fn close(self) -> Result<(), Error> {
        Ok(self.0.close(CloseCode::default(), None).await?)
    }
}
