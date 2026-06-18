use roblox_api::Paging;
use roblox_api::api::private_messages::v1::MessageTab;

test_endpoint!(unread_count, [private_messages::v1], unread_count);
test_endpoint!(messages, [private_messages::v1], messages(MessageTab::Inbox, Paging::new(Some(&0u64.to_string()), Some(100), None)));
test_endpoint!(announcements, [private_messages::v1], announcements);
