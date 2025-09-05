use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::groups, client::Client};

const BHOP_GROUP_ID: u64 = 6980477;

#[tokio::test]
async fn information() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::information(&mut client, BHOP_GROUP_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn users() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::users(&mut client, BHOP_GROUP_ID, Paging::default())
        .await
        .unwrap();
}
