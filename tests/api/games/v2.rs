use roblox_api::{Paging, SortOrder};

const USER_ID: u64 = 3139503587;
const BHOP_GROUP_ID: u64 = 6980477;
const BHOP_UNIVERSE_ID: u64 = 1861504796;

test_endpoint!(universe_media, [games::v2], universe_media(BHOP_UNIVERSE_ID, true));
test_endpoint!(group_games_v2, [games::v2], group_games_v2(BHOP_GROUP_ID, 1, Paging::default()));
test_endpoint!(user_games, [games::v2], user_games(USER_ID, 2, Paging::default()));
test_endpoint!(user_favorited_games, [games::v2], user_favorited_games(USER_ID, 2, Paging::new(None, None, Some(SortOrder::Descending))));
