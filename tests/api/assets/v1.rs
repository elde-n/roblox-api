use dotenvy_macro::dotenv;
use roblox_api::{api::assets, client::Client};

const ASSET_ID: u64 = 6340213;

#[tokio::test]
async fn asset() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    assets::v1::asset(&mut client, ASSET_ID).await.unwrap();
}
