use dotenvy_macro::dotenv;
use roblox_api::{api::user_blocking, client::Client};

#[tokio::test]
async fn is_blocked() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    user_blocking::v1::is_blocked(&mut client, 1).await.unwrap();
}
