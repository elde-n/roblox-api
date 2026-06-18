use roblox_api::DateTime;

test_endpoint!(recommended_usernames, [auth::v1], recommended_usernames_from_display_name("TestUser", DateTime::from_ymd(2020, 1, 1)));
