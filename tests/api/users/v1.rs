use dotenvy_macro::dotenv;
use roblox_api::{api::users, client::Client};

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
