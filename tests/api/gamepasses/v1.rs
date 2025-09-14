use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::gamepasses, client::Client};

const USER_ID: u64 = 3139503587;
const BHOP_CHAT_EFFECTS_GAMEPASS_ID: u64 = 10613803;

#[tokio::test]
async fn details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let details = gamepasses::v1::details(&mut client, BHOP_CHAT_EFFECTS_GAMEPASS_ID)
        .await
        .unwrap();

    assert_eq!(details.id, BHOP_CHAT_EFFECTS_GAMEPASS_ID);
}

#[tokio::test]
async fn product_information() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let information =
        gamepasses::v1::product_information(&mut client, BHOP_CHAT_EFFECTS_GAMEPASS_ID)
            .await
            .unwrap();

    assert_eq!(information.id, BHOP_CHAT_EFFECTS_GAMEPASS_ID);
}

#[tokio::test]
async fn user_gamepasses() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    gamepasses::v1::user_gamepasses(&mut client, USER_ID, Paging::default())
        .await
        .unwrap();
}
