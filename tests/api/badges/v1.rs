use dotenvy_macro::dotenv;
use roblox_api::{Paging, api::badges, client::Client};

const USER_ID: u64 = 3139503587;

const BHOP_UNIVERSE_ID: u64 = 1861504796;
const BHOP_TIME_BADGE_ID: u64 = 2124615090;
const BHOP_ANNOYING_BADGE_ID: u64 = 2124614454;

#[tokio::test]
async fn information() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let info = badges::v1::information(&mut client, BHOP_TIME_BADGE_ID)
        .await
        .unwrap();

    assert_eq!(info.id, BHOP_TIME_BADGE_ID);
}

#[tokio::test]
async fn universe_badges() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result =
        badges::v1::universe_badges(&mut client, BHOP_UNIVERSE_ID, None, Paging::default())
            .await
            .unwrap();

    assert!(result.badges.len() > 0);
}

#[tokio::test]
async fn user_badges() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result = badges::v1::user_badges(&mut client, USER_ID, Paging::default())
        .await
        .unwrap();

    assert!(result.badges.len() > 0);
}

// Our test account doesn't have any way to automatically get badges to test
//#[tokio::test]
//async fn authenticated_remove() {
//    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
//    client.ensure_token().await.unwrap();
//    badges::v1::authenticated_remove(&mut client, BHOP_ANNOYING_BADGE_ID)
//        .await
//        .unwrap();
//}
