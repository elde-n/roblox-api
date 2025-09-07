use dotenvy_macro::dotenv;
use roblox_api::{Paging, SortOrder, api::games, client::Client};

const USER_ID: u64 = 3139503587;

const BHOP_GROUP_ID: u64 = 6980477;
//const BHOP_PLACE_ID: u64 = 5315046213;
const BHOP_UNIVERSE_ID: u64 = 1861504796;

#[tokio::test]
async fn universe_media() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v2::universe_media(&mut client, BHOP_UNIVERSE_ID, true)
        .await
        .unwrap();
}

#[tokio::test]
async fn group_games_v2() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v2::group_games_v2(&mut client, BHOP_GROUP_ID, 1, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn user_games() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v2::user_games(&mut client, USER_ID, 2, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn user_favorited_games() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    games::v2::user_favorited_games(
        &mut client,
        USER_ID,
        2,
        Paging::new(None, None, Some(SortOrder::Descending)),
    )
    .await
    .unwrap();
}
