use roblox_api::{api::economy, client::Client};

#[tokio::test]
async fn details() {
    let mut client = Client::default();

    client.ensure_token().await.unwrap();
    economy::v2::details(&mut client, 48474313).await.unwrap();
}
