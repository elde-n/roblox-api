use roblox_api::Paging;

const USER_ID: u64 = 3139503587;
const BHOP_CHAT_EFFECTS_GAMEPASS_ID: u64 = 10613803;

test_endpoint!(details, [gamepasses::v1], details(BHOP_CHAT_EFFECTS_GAMEPASS_ID) => |details| {
    assert_eq!(details.id, BHOP_CHAT_EFFECTS_GAMEPASS_ID);
});

test_endpoint!(product_information, [gamepasses::v1], product_information(BHOP_CHAT_EFFECTS_GAMEPASS_ID) => |info| {
    assert_eq!(info.id, BHOP_CHAT_EFFECTS_GAMEPASS_ID);
});

test_endpoint!(user_gamepasses, [gamepasses::v1], user_gamepasses(USER_ID, Paging::default()));
