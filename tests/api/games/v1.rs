use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::games, client::Client};

const BHOP_PLACE_ID: u64 = 5315046213;
const BHOP_UNIVERSE_ID: u64 = 1861504796;

#[tokio::test]
async fn batch_place_details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::batch_place_details(&mut client, &[BHOP_PLACE_ID])
        .await
        .unwrap();
}

#[tokio::test]
async fn servers() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::servers(&mut client, BHOP_PLACE_ID, 0, false, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn private_servers() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::private_servers(&mut client, BHOP_PLACE_ID, false, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn universe_favorite_count() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::universe_favorite_count(&mut client, BHOP_UNIVERSE_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn universe_votes() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::universe_votes(&mut client, &[BHOP_UNIVERSE_ID])
        .await
        .unwrap();
}

#[tokio::test]
async fn universe_gamepasses() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v1::universe_gamepasses(&mut client, BHOP_UNIVERSE_ID, Paging::default())
        .await
        .unwrap();
}
