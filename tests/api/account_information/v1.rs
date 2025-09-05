use dotenvy_macro::dotenv;
use roblox_api::{api::account_information, client::Client};

#[tokio::test]
async fn roblox_badges() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    account_information::v1::roblox_badges(&mut client, 1)
        .await
        .unwrap();
}
