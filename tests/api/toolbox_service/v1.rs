use roblox_api::{AssetTypeId, Paging};

test_endpoint!(item_details, [toolbox_service::v1], item_details(&[47433]) => |items| {
    assert!(!items.is_empty());
});

test_endpoint!(creations, [toolbox_service::v1], creations(11117566207, AssetTypeId::Model, Paging::default()) => |creations| {
    assert!(creations.results >= 0, "creations count should be non-negative");
});
