use dotenvy_macro::dotenv;

use roblox_api::{
    api::{economy, users},
    client::Client,
};

const LOL_GROUP_ID: u64 = 2;

#[tokio::test]
async fn currency() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    economy::v1::currency(&mut client).await.unwrap();
}

#[tokio::test]
async fn currency_from_user_id() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    let authenticated = users::v1::authenticated_details(&mut client).await.unwrap();
    economy::v1::currency_from_user_id(&mut client, authenticated.id)
        .await
        .unwrap();
}

#[tokio::test]
async fn currency_from_group_id() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    economy::v1::currency_from_group_id(&mut client, LOL_GROUP_ID)
        .await
        .unwrap();
}
