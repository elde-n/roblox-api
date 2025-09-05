use roblox_api::{api::economy, client::Client};

#[tokio::test]
async fn details() {
    let mut client = Client::default();
    economy::v2::details(&mut client, 48474313).await.unwrap();
}
