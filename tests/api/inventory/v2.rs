use roblox_api::{AssetTypeId, Paging};

const USER_ID: u64 = 3139503587;

test_endpoint!(user_owned_assets, [inventory::v2], user_owned_assets(USER_ID, AssetTypeId::Hat, Paging::default()) => |result| {
    assert!(!result.assets.is_empty());
});
