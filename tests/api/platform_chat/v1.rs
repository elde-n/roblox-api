use dotenvy_macro::dotenv;
use roblox_api::{
    Paging,
    api::platform_chat::{self, v1::ConversationCreateRequest},
    client::Client,
};

#[tokio::test]
async fn conversation_metadata() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    platform_chat::v1::conversation_metadata(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_conversations() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
        .await
        .unwrap();
}

#[tokio::test]
async fn conversation_participants_metadata() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::conversations_participant_metadata(
        &mut client,
        &[conversation.id.as_ref().unwrap().as_str()],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn conversations() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::conversations(&mut client, &[conversation.id.as_ref().unwrap().as_str()])
        .await
        .unwrap();
}

#[tokio::test]
async fn conversation_messages() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();
    platform_chat::v1::conversation_messages(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn send_messages_in_conversation() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();
    client.ensure_token().await.unwrap();

    platform_chat::v1::send_messages_in_conversation(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
        &["Hello"],
    )
    .await
    .unwrap();

    platform_chat::v1::send_messages_in_conversation(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
        &["world"],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn update_typing_status_in_conversation() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::update_typing_status_in_conversation(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn add_users_to_conversation() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::add_users_to_conversation(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
        &[1],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn remove_users_from_conversation() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::remove_users_from_conversation(
        &mut client,
        conversation.id.as_ref().unwrap().as_str(),
        &[1],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn create_conversations() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    platform_chat::v1::create_conversations(
        &mut client,
        &[ConversationCreateRequest {
            name: "name".to_string(),
            users: vec![1, 2],
        }],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn rename_conversations() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::rename_conversations(
        &mut client,
        &[conversation.id.as_ref().unwrap().as_str()],
        &["test"],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn mark_conversations_as_read() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        platform_chat::v1::user_conversations(&mut client, Paging::new(None, Some(100), None))
            .await
            .unwrap();

    let conversation = result.conversations.first().unwrap();

    client.ensure_token().await.unwrap();
    platform_chat::v1::mark_conversations_as_read(
        &mut client,
        &[conversation.id.as_ref().unwrap().as_str()],
    )
    .await
    .unwrap();
}
