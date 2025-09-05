use dotenvy_macro::dotenv;
use roblox_api::{AssetTypeId, Paging, api::inventory, client::Client};

const USER_ID: u64 = 3139503587;

#[tokio::test]
async fn user_owned_assets() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    inventory::v2::user_owned_assets(&mut client, USER_ID, AssetTypeId::Hat, Paging::default())
        .await
        .unwrap();
}
