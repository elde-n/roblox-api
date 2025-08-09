use dotenvy_macro::dotenv;

use roblox_api::{Currency, api::economy, client::Client};

#[tokio::test]
async fn purchase() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();

    let asset_details = economy::v2::details(&mut client, 48474313).await.unwrap();
    economy::v1::purchase(
        &mut client,
        asset_details.product_id,
        0,
        Currency::default(),
        Some(asset_details.creator.id),
    )
    .await
    .unwrap();
}
