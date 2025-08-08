use std::time::Duration;

use roblox_api::{
    api::{auth, auth::v1::LoginType, auth_token_service, auth_token_service::v1::LoginStatus},
    client::Client,
};

#[tokio::main]
async fn main() {
    let mut client = Client::default();
    client.ensure_token().await.unwrap();

    let token = auth_token_service::v1::login_create(&mut client)
        .await
        .unwrap();

    client.ensure_token().await.unwrap();
    loop {
        let status =
            auth_token_service::v1::login_status(&mut client, &token.code, &token.private_key)
                .await
                .unwrap();

        if status.status == LoginStatus::Validated {
            break;
        } else {
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    auth::v1::login(
        &mut client,
        &token.code,
        &token.private_key,
        LoginType::AuthToken,
    )
    .await
    .unwrap();
}
