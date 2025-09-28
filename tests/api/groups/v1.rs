use dotenvy_macro::dotenv;
use roblox_api::{
    ApiError, Error, Paging,
    api::{groups, users},
    client::Client,
};

const USER_ID: u64 = 3139503587;

const BHOP_GROUP_ID: u64 = 6980477;

const ROBLOX_GROUP_ID: u64 = 7;
const ROBLOX_GROUP_GUEST_ROLE_ID: u64 = 260;

#[tokio::test]
async fn information() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::information(&mut client, BHOP_GROUP_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn membership() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let membership = groups::v1::membership(&mut client, BHOP_GROUP_ID, false)
        .await
        .unwrap();

    assert_eq!(membership.id, BHOP_GROUP_ID);
}

#[tokio::test]
async fn name_history() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::name_history(&mut client, BHOP_GROUP_ID)
        .await
        .unwrap();
}

#[tokio::test]
async fn pending_join_requests() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::pending_join_requests(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn roles() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::roles(&mut client, BHOP_GROUP_ID).await.unwrap();
}

#[tokio::test]
async fn user_roles() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::user_roles(&mut client, USER_ID).await.unwrap();
}

#[tokio::test]
async fn roleset_permissions() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let permissions =
        groups::v1::roleset_permissions(&mut client, ROBLOX_GROUP_ID, ROBLOX_GROUP_GUEST_ROLE_ID)
            .await
            .unwrap();

    assert_eq!(permissions.id, ROBLOX_GROUP_ID);
    assert_eq!(permissions.role.id, ROBLOX_GROUP_GUEST_ROLE_ID);
    assert_eq!(permissions.role.rank, 0);
}

#[tokio::test]
async fn users() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::users(&mut client, BHOP_GROUP_ID, Paging::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn wall_posts() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    groups::v1::wall_posts(&mut client, BHOP_GROUP_ID, Paging::default())
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
