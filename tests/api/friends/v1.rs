use roblox_api::Paging;

const USER_ID: u64 = 3139503587;

test_endpoint!(friend_requests_count, [friends::v1], friend_requests_count);
test_endpoint!(new_friend_requests_count, [friends::v1], new_friend_requests_count);
test_endpoint!(user_friends_count, [friends::v1], user_friends_count(USER_ID));
test_endpoint!(user_followings_count, [friends::v1], user_followings_count(USER_ID));
test_endpoint!(user_followers_count, [friends::v1], user_followers_count(USER_ID));
test_endpoint!(following_status, [friends::v1], following_status(&[USER_ID]));
test_endpoint!(friend_requests, [friends::v1], friend_requests(Paging::default()));
test_endpoint!(user_followers, [friends::v1], user_followers(USER_ID));
test_endpoint!(user_followings, [friends::v1], user_followings(USER_ID));
test_endpoint!(user_friends_online, [friends::v1], user_friends_online(USER_ID));
test_endpoint!(user_friends_find, [friends::v1], user_friends_find(USER_ID, Paging::default()));
test_endpoint!(user_friends_search, [friends::v1], user_friends_search(USER_ID, "Roblox", Paging::default()));

test_endpoint!(user_friend_statuses, [friends::v1], user_friend_statuses(USER_ID, &[1u64]) => |statuses| {
    assert!(!statuses.is_empty());
});
