
const USER_ID: u64 = 3139503587;

test_endpoint!(is_premium, [premium_features::v1], is_premium(USER_ID) => |ok| {
    assert!(ok);
});
