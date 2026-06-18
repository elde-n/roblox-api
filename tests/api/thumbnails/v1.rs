use roblox_api::api::thumbnails::v1::{
    ReturnPolicy, ThumbnailBatchRequest, ThumbnailFormat, ThumbnailRequestType, ThumbnailSize,
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

test_endpoint_noauth!(assets, [thumbnails::v1], assets(&[47433u64, 187789986], ThumbnailSize::S420x420, ThumbnailFormat::default(), ReturnPolicy::default(), false) => |thumbnails| {
    assert!(!thumbnails.is_empty());
});

test_endpoint_noauth!(badge_icons, [thumbnails::v1], badge_icons(&[2124615090u64], ThumbnailSize::S150x150, ThumbnailFormat::default(), false) => |thumbnails| {
    assert!(!thumbnails.is_empty());
});

test_endpoint_noauth!(bundles, [thumbnails::v1], bundles(&[175772208088820u64], ThumbnailSize::S420x420, ThumbnailFormat::default(), false) => |thumbnails| {
    assert!(!thumbnails.is_empty());
});

test_endpoint_noauth!(batch, [thumbnails::v1], batch(vec![ThumbnailBatchRequest {
    id: 3139503587,
    request_id: "",
    token: "",
    alias: "",
    kind: ThumbnailRequestType::AvatarHeadShot,
    size: ThumbnailSize::S420x420,
    format: ThumbnailFormat::default(),
    circular: true,
}]) => |thumbnails| {
    assert!(!thumbnails.is_empty());
});
