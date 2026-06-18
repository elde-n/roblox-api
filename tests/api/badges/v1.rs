use roblox_api::Paging;

const USER_ID: u64 = 3139503587;
const BHOP_UNIVERSE_ID: u64 = 1861504796;
const BHOP_TIME_BADGE_ID: u64 = 2124615090;

test_endpoint!(information, [badges::v1], information(BHOP_TIME_BADGE_ID) => |info| {
    assert_eq!(info.id, BHOP_TIME_BADGE_ID);
    assert!(!info.name.is_empty());
});

test_endpoint!(universe_badges, [badges::v1], universe_badges(BHOP_UNIVERSE_ID, None, Paging::default()) => |result| {
    assert!(!result.badges.is_empty());
});

test_endpoint!(user_badges, [badges::v1], user_badges(USER_ID, Paging::default()) => |result| {
    assert!(!result.badges.is_empty());
});
