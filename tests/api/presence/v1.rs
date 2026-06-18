
test_endpoint!(presence, [presence::v1], presence(&[1u64, 2, 3]) => |result| {
    assert_eq!(result.len(), 3);
    assert_eq!(result.first().unwrap().id, 1);
});
