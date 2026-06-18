
const ASSET_IDS: &[u64] = &[144075659, 144076358];

test_endpoint!(assets, [develop::v1], assets(ASSET_IDS) => |assets| {
    assert_eq!(assets.len(), 2);
});
