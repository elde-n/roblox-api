use dotenvy_macro::dotenv;
use roblox_api::{
    api::thumbnails::{
        self,
        v1::{ReturnPolicy, ThumbnailFormat, ThumbnailSize},
    },
    client::Client,
};

#[tokio::test]
async fn ratelimit() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    // this api should have ratelimits
    thumbnails::v1::assets(
        &mut client,
        &[47433],
        ThumbnailSize::S420x420,
        ThumbnailFormat::default(),
        ReturnPolicy::default(),
        false,
    )
    .await
    .unwrap();

    client.ratelimits().await.unwrap();
}
