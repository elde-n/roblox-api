use dotenvy_macro::dotenv;
use roblox_api::{
    ApiError, Error, Paging,
    api::{groups, users},
    client::Client,
};

const ROBLOX_GROUP_ID: u64 = 7;
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

#[tokio::test]
async fn join() -> Result<(), Error> {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    let result = groups::v1::join(&mut client, ROBLOX_GROUP_ID).await;

    if let Err(Error::ApiError(ApiError::AlreadyInGroup)) = result {
        Ok(())
    } else {
        result
    }
}

#[tokio::test]
async fn join_request() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();

    // ensure we are not in the group join queue
    let details = users::v1::authenticated_details(&mut client).await.unwrap();
    groups::v1::remove_join_request(&mut client, BHOP_GROUP_ID, details.id)
        .await
        .unwrap();

    groups::v1::join(&mut client, BHOP_GROUP_ID).await.unwrap();
}
