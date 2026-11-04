use roblox_api::api::client_settings::v2::{self as cs, BinaryType};

const BINARY_TYPE: BinaryType = BinaryType::MacPlayer;
const APPLICATION: &str = "MacDesktopClient";

test_endpoint!(client_version, [client_settings::v2], client_version(BINARY_TYPE, None, None) => |resp| {
    assert!(!resp.version.is_empty());
});

test_endpoint!(client_version_with_channel, [client_settings::v2], client_version(BINARY_TYPE, Some("live"), None) => |resp| {
    assert!(!resp.version.is_empty());
});

test_endpoint!(client_version_with_token, [client_settings::v2], client_version(BINARY_TYPE, Some("live"), Some("invalid")) => |resp| {
    assert!(!resp.version.is_empty());
});

test_endpoint!(user_channel, [client_settings::v2], user_channel(BINARY_TYPE) => |resp| {
    assert!(!resp.channel_name.is_empty());
});

test_endpoint!(compression_dictionaries, [client_settings::v2], compression_dictionaries() => |resp| {
    assert!(!resp.is_empty());
    assert!(!resp[0].sha256.is_empty());
});

test_endpoint!(settings, [client_settings::v2], settings(APPLICATION, None) => |resp| {
    assert!(!resp.application_settings.is_empty());
});

test_endpoint!(settings_with_bucket, [client_settings::v2], settings(APPLICATION, Some("zcanary")) => |resp| {
    assert!(!resp.application_settings.is_empty());
});

#[tokio::test]
async fn ota_version() {
    let mut client = crate::api::client();
    let version = cs::client_version(&mut client, BINARY_TYPE, None, None)
        .await
        .unwrap()
        .version;

    let ota = cs::ota_version(&mut client, BINARY_TYPE, &version, None, None, None)
        .await
        .unwrap();

    assert!(!ota.is_empty());
}

#[tokio::test]
async fn compression_dictionary() {
    let mut client = crate::api::client();

    for hash in cs::compression_dictionaries(&mut client).await.unwrap() {
        if let Ok(dictionary) = cs::compression_dictionary(&mut client, &hash.sha256).await {
            assert!(!dictionary.is_empty());
            return;
        }
    }

    panic!("no fetchable compression dictionary");
}

#[tokio::test]
async fn settings_compressed() {
    let mut client = crate::api::client();

    for hash in cs::compression_dictionaries(&mut client).await.unwrap() {
        if let Ok(compressed) =
            cs::settings_compressed(&mut client, APPLICATION, &hash.sha256, None).await
        {
            assert_eq!(&compressed[..4], &[0x28, 0xB5, 0x2F, 0xFD]);
            return;
        }
    }

    panic!("no fetchable compressed settings");
}
