use roblox_api::Paging;

test_endpoint!(unread_count, [notifications::v2], unread_count);
test_endpoint!(recent, [notifications::v2], recent(Paging::default()));
test_endpoint!(clear_unread, [notifications::v2], clear_unread);
