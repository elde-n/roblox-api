use dotenvy_macro::dotenv;
use roblox_api::{api::friends, client::Client};

const USER_ID: u64 = 3139503587;

#[tokio::test]
async fn friend_requests_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::friend_requests_count(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn new_friend_requests_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::new_friend_requests_count(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_friends_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_friends_count(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_followings_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_followings_count(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_followers_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_followers_count(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn following_status() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    friends::v1::following_status(&mut client, &[USER_ID])
        .await
        .unwrap();
}
