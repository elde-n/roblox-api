
test_endpoint_noauth!(details, [economy::v2], details(48474313) => |info| {
    assert!(!info.name.is_empty());
});
