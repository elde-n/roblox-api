use roblox_api::{
    api::thumbnails::{
        self,
        v1::{
            ReturnPolicy, ThumbnailBatchRequest, ThumbnailFormat, ThumbnailRequestType,
            ThumbnailSize,
        },
    },
    client::{Client, Cookie},
};

#[tokio::main]
async fn main() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());

    client.ensure_token().await.unwrap();

    let thumbnails = thumbnails::v1::assets(
        &mut client,
        &[47433, 187789986],
        ThumbnailSize::S420x420,
        ThumbnailFormat::default(),
        ReturnPolicy::default(),
        false,
    )
    .await
    .unwrap();

    for thumbnail in &thumbnails {
        println!("{:?}", thumbnail);
    }

    let badge_icons = thumbnails::v1::badge_icons(
        &mut client,
        &[2124615090],
        ThumbnailSize::S150x150,
        ThumbnailFormat::default(),
        false,
    )
    .await
    .unwrap();

    for thumbnail in &badge_icons {
        println!("{:?}", thumbnail);
    }

    let bundles = thumbnails::v1::bundles(
        &mut client,
        &[175772208088820],
        ThumbnailSize::S420x420,
        ThumbnailFormat::default(),
        false,
    )
    .await
    .unwrap();

    for thumbnail in &bundles {
        println!("{:?}", thumbnail);
    }

    let batch = thumbnails::v1::batch(
        &mut client,
        vec![ThumbnailBatchRequest {
            id: 3139503587,
            request_id: "",
            token: "",
            alias: "",
            kind: ThumbnailRequestType::AvatarHeadShot,
            size: ThumbnailSize::S420x420,
            format: ThumbnailFormat::default(),
            circular: true,
        }],
    )
    .await
    .unwrap();

    for thumbnail in &batch {
        println!("{:?}", thumbnail);
    }
}
