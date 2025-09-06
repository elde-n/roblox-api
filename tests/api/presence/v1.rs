use dotenvy_macro::dotenv;
use roblox_api::{api::presence, client::Client};

#[tokio::test]
async fn presence() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    presence::v1::presence(&mut client, &[1, 2, 3])
        .await
        .unwrap();
}
