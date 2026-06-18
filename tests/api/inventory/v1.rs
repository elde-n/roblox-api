use roblox_api::{Paging, api::inventory::v1::ItemType};

test_endpoint!(user_owns_assets, [inventory::v1], user_owns_assets(3139503587, 4391384843, ItemType::Asset, Paging::default()) => |result| {
    assert!(!result.assets.is_empty());
});

test_endpoint!(user_owned_collectibles, [inventory::v1], user_owned_collectibles(3139503587, None, Paging::default()) => |result| {
    assert!(!result.assets.is_empty());
});
