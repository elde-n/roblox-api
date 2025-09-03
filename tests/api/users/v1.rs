use dotenvy_macro::dotenv;
use roblox_api::{Paging, SortOrder, api::users, client::Client};

#[tokio::test]
async fn authenticated_details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::authenticated_details(&mut client).await.unwrap();
}

#[tokio::test]
async fn birthdate() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::birthdate(&mut client).await.unwrap();
}

#[tokio::test]
async fn gender() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::gender(&mut client).await.unwrap();
}

#[tokio::test]
async fn description() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::description(&mut client).await.unwrap();
}

#[tokio::test]
async fn validate_display_name_by_id() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();

    let authenticated = users::v1::authenticated_details(&mut client).await.unwrap();
    users::v1::validate_display_name_by_id(&mut client, authenticated.id, "エルデン")
        .await
        .unwrap();
}

#[tokio::test]
async fn user_details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::user_details(&mut client, 1).await.unwrap();
}

#[tokio::test]
async fn user_username_history() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::user_username_history(
        &mut client,
        85382088,
        Paging::new(None, Some(100), Some(SortOrder::Ascending)),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn users_by_id() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::users_by_id(&mut client, &[1, 2, 3, 4], false)
        .await
        .unwrap();
}

#[tokio::test]
async fn users_by_name() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    users::v1::users_by_name(&mut client, &["Roblox", "test", "word"], false)
        .await
        .unwrap();
}
