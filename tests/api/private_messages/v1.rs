use dotenvy_macro::dotenv;
use roblox_api::{
    api::private_messages::{self, v1::MessageTab},
    client::Client,
};

#[tokio::test]
async fn unread_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    private_messages::v1::unread_count(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn messages() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    private_messages::v1::messages(&mut client, MessageTab::Inbox, 0, 100)
        .await
        .unwrap();
}

#[tokio::test]
async fn announcements() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    private_messages::v1::announcements(&mut client)
        .await
        .unwrap();
}
