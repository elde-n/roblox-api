use roblox_api::Paging;

const BHOP_PLACE_ID: u64 = 5315046213;
const BHOP_UNIVERSE_ID: u64 = 1861504796;

test_endpoint!(batch_place_details, [games::v1], batch_place_details(&[BHOP_PLACE_ID]) => |places| {
    assert!(!places.is_empty());
});
test_endpoint!(servers, [games::v1], servers(BHOP_PLACE_ID, 0, false, Paging::default()));
test_endpoint!(private_servers, [games::v1], private_servers(BHOP_PLACE_ID, false, Paging::default()));
test_endpoint!(universe_favorite_count, [games::v1], universe_favorite_count(BHOP_UNIVERSE_ID) => |count| {
    assert!(count >= 0);
});
test_endpoint!(universe_votes, [games::v1], universe_votes(&[BHOP_UNIVERSE_ID]) => |votes| {
    assert!(votes.len() > 0);
});
test_endpoint!(universe_gamepasses, [games::v1], universe_gamepasses(BHOP_UNIVERSE_ID, Paging::default()));
