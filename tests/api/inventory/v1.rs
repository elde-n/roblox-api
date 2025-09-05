use dotenvy_macro::dotenv;
use roblox_api::{
    Paging,
    api::inventory::{self, v1::ItemType},
    client::Client,
};

#[tokio::test]
async fn user_owned_assets() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    inventory::v1::user_owns_assets(
        &mut client,
        3139503587,
        4391384843,
        ItemType::Asset,
        Paging::default(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn user_owned_collectibles() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();
    inventory::v1::user_owned_collectibles(&mut client, 3139503587, None, Paging::default())
        .await
        .unwrap();
}
