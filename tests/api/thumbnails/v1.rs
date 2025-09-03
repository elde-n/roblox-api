use roblox_api::{
    api::thumbnails::{
        self,
        v1::{
            ReturnPolicy, ThumbnailBatchRequest, ThumbnailFormat, ThumbnailRequestType,
            ThumbnailSize,
        },
    },
    client::Client,
};

#[test]
fn thumbnail_size_from_str() {
    assert_eq!(
        ThumbnailSize::try_from("100x100").unwrap(),
        ThumbnailSize::S100x100
    );
}

#[test]
fn thumbnail_request_type_from_str() {
    assert_eq!(
        ThumbnailRequestType::try_from("GameThumbnail").unwrap(),
        ThumbnailRequestType::GameThumbnail
    );
}

#[tokio::test]
async fn assets() {
    let mut client = Client::default();

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

    thumbnails.get(0).unwrap();
}

#[tokio::test]
async fn badge_icons() {
    let mut client = Client::default();

    let thumbnails = thumbnails::v1::badge_icons(
        &mut client,
        &[2124615090],
        ThumbnailSize::S150x150,
        ThumbnailFormat::default(),
        false,
    )
    .await
    .unwrap();

    thumbnails.get(0).unwrap();
}

#[tokio::test]
async fn bundles() {
    let mut client = Client::default();

    let thumbnails = thumbnails::v1::bundles(
        &mut client,
        &[175772208088820],
        ThumbnailSize::S420x420,
        ThumbnailFormat::default(),
        false,
    )
    .await
    .unwrap();

    thumbnails.get(0).unwrap();
}

#[tokio::test]
async fn batch() {
    let mut client = Client::default();

    let thumbnails = thumbnails::v1::batch(
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

    thumbnails.get(0).unwrap();
}
