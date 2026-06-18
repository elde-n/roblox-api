
const ASSET_ID: u64 = 6340213;

test_endpoint!(asset, [assets::v1], asset(ASSET_ID) => |asset| {
    assert!(!asset.name.is_empty());
});
