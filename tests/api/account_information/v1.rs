
test_endpoint!(roblox_badges, [account_information::v1], roblox_badges(1) => |badges| {
    assert!(!badges.is_empty());
});
