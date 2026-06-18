
test_endpoint_noauth!(server_nonce, [hba_service::v1], server_nonce() => |nonce| {
    assert!(!nonce.is_empty());
});
