use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::notifications, client::Client};

#[tokio::test]
async fn unread_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    notifications::v2::unread_count(&mut client).await.unwrap();
}

#[tokio::test]
async fn recent() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    notifications::v2::recent(&mut client, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn clear_unread() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    notifications::v2::clear_unread(&mut client).await.unwrap();
}
