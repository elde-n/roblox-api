use dotenvy_macro::dotenv;
use roblox_api::{api::premium_features, client::Client};

const USER_ID: u64 = 3139503587;

#[tokio::test]
async fn is_premium() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    premium_features::v1::is_premium(&mut client, USER_ID)
        .await
        .unwrap();
}
