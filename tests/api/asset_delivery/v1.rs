
const ASSET_ID: u64 = 144075659;

test_endpoint_noauth!(asset, [asset_delivery::v1], asset(ASSET_ID) => |data| {
    assert!(!data.is_empty());
});
