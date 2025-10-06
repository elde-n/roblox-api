use dotenvy_macro::dotenv;
use roblox_api::{api::auth_token_service, client::Client};

#[tokio::test]
async fn login_create() {
    let mut client = Client::default();
    auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();
}

#[tokio::test]
async fn login_status() {
    let mut client = Client::default();

    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    client.ensure_token().await.unwrap();
    auth_token_service::v1::login_status(&mut client, &token.code, &token.private_key)
        .await
        .unwrap();
}

#[tokio::test]
async fn login_cancel() {
    let mut client = Client::default();

    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    auth_token_service::v1::login_cancel(&mut client, &token.code)
        .await
        .unwrap();
}

#[tokio::test]
async fn inspect_code() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    client.ensure_token().await.unwrap();
    auth_token_service::v1::inspect_code(&mut client, &token.code)
        .await
        .unwrap();
}

#[tokio::test]
async fn validate_code() {
    let mut client = Client::default();
    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();

    auth_token_service::v1::inspect_code(&mut client, &token.code)
        .await
        .unwrap();

    auth_token_service::v1::validate_code(&mut client, &token.code)
        .await
        .unwrap();
}

#[tokio::test]
async fn qr_code_image() {
    let mut client = Client::default();
    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    let bytes = auth_token_service::v1::qr_code_image(&mut client, &token.private_key, &token.code)
        .await
        .unwrap();

    assert!(bytes.len() > 0);
}
