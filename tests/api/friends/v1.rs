use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::friends, client::Client};

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

#[tokio::test]
async fn friend_requests() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::friend_requests(&mut client, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn user_followers() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_followers(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_followings() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_followings(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_friends_online() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_friends_online(&mut client, USER_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn user_friends_find() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_friends_find(&mut client, USER_ID, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn user_friends_search() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    friends::v1::user_friends_search(&mut client, USER_ID, "Roblox", Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn user_friend_statuses() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let statuses = friends::v1::user_friend_statuses(&mut client, USER_ID, &[1])
        .await
        .unwrap();

    assert_eq!(statuses.first().unwrap().id, 1);
}
