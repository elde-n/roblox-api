use dotenvy_macro::dotenv;
use roblox_api::{
    Paging,
    api::avatar::{
        self,
        v1::{AvatarScales, AvatarType, BodyColors},
    },
    client::Client,
};

const USER_ID: u64 = 3139503587;

#[tokio::test]
async fn user_avatar() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    avatar::v1::user_avatar(&mut client, 1).await.unwrap();
}

#[tokio::test]
async fn user_currently_wearing() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    avatar::v1::user_currently_wearing(&mut client, 1)
        .await
        .unwrap();
}

#[tokio::test]
async fn set_currently_wearing() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();
    avatar::v1::avatar_set_wearing_assets(&mut client, vec![1])
        .await
        .unwrap();
}

#[tokio::test]
async fn avatar_set_type() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();
    avatar::v1::avatar_set_type(&mut client, AvatarType::R6)
        .await
        .unwrap();
}

#[tokio::test]
async fn avatar_set_body_colors() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();
    avatar::v1::avatar_set_body_colors(
        &mut client,
        BodyColors {
            head: 125,
            torso: 125,
            right_arm: 125,
            left_arm: 125,
            right_leg: 125,
            left_leg: 125,
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn avatar_set_scales() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();
    avatar::v1::avatar_set_scales(
        &mut client,
        AvatarScales {
            height: 1.0,
            width: 1.0,
            head: 1.0,
            depth: 1.0,
            proportion: 1.0,
            body_type: 1.0,
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn user_outfits() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    avatar::v1::user_outfits(&mut client, 1, Paging::default(), None)
        .await
        .unwrap();
}

#[tokio::test]
async fn outfit_details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    let result = avatar::v1::user_outfits(&mut client, USER_ID, Paging::default(), None)
        .await
        .unwrap();
    let outfit = result.outfits.first().unwrap();

    let details = avatar::v1::outfit_details(&mut client, outfit.id)
        .await
        .unwrap();

    assert_eq!(outfit.id, details.id);
}

#[tokio::test]
async fn remove_outfit() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();
    avatar::v1::remove_outfit(&mut client, u64::MAX)
        .await
        .unwrap();
}
