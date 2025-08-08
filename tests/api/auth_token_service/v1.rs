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
