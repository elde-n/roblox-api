
test_endpoint!(currency, [economy::v1], currency);
test_endpoint!(currency_from_user_id, [economy::v1], currency_from_user_id(1));
test_endpoint!(currency_from_group_id, [economy::v1], currency_from_group_id(2));
