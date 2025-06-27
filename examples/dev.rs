use roblox_api::{
    AssetTypeId,
    api::{
        assets::{
            self,
            v1::{AssetType, CreationContext, Creator},
        },
        data, develop, users,
    },
    client::{Client, Cookie},
};

#[tokio::main]
async fn main() {
    let cookie: String = std::fs::read_to_string(".cookie")
        .unwrap()
        .split_whitespace()
        .collect();

    let mut client = Client::from_cookie(Cookie::from(cookie.as_str()));
    let authenticted = users::v1::authenticated_details(&mut client).await.unwrap();

    let bytes = &[
        0x3c, 0x72, 0x6f, 0x62, 0x6c, 0x6f, 0x78, 0x21, // signature "<roblox!" (u64)
        0x89, 0xff, 0x0d, 0x0a, 0x1a, 0x0a, // magic (u32 + u16)
        0x00, 0x00, //version (u16)
        0x00, 0x00, 0x00, 0x00, // class count (u32)
        0x00, 0x00, 0x00, 0x00, // inst count (u32)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserved (u64)
        0x50, 0x52, 0x4e, 0x54, // chunk.name "PRNT" (u32)
        0x00, 0x00, 0x00, 0x00, // chunk.compressed_length (u32)
        0x00, 0x05, 0x00, 0x00, // chunk.after_compression_length (u32),
        0x00, 0x00, 0x00, 0x00, // chunk.reserved (u32)
        0x00, // parent.version (u8),
        0x00, 0x00, 0x00, 0x00, // parent.instance_count (u32),
        //
        0x53, 0x49, 0x47, 0x4e, // chunk.name "SIGN" (u32)
        0x00, 0x00, 0x00, 0x00, // chunk.compressed_length (u32)
        0x00, 0x0e, 0x00, 0x00, // chunk.after_compression_length (u32),
        0x00, 0x00, 0x00, 0x00, // chunk.reserved (u32)
        0x65, 0x6c, 0x64, 0x65, 0x6e, 0x20, 0x77, 0x61, 0x73, 0x20, 0x68, 0x65, 0x72,
        0x65, // my signature (u64 + u32 + u16)
        //
        0x45, 0x4e, 0x44, 0x00, // chunk.name "END" (u32)
        0x00, 0x00, 0x00, 0x00, // chunk.compressed_length (u32)
        0x00, 0x09, 0x00, 0x00, // chunk.after_compression_length (u32),
        0x00, 0x00, 0x00, 0x00, // chunk.reserved (u32)
        0x3c, 0x2f, 0x72, 0x6f, 0x62, 0x6c, 0x6f, 0x78,
        0x3e,
        // end signature "</roblox>" (u64 + u8)
    ];

    // ensure token
    let _ = data::upload(
        &mut client,
        None,
        "",
        "",
        AssetTypeId::Model,
        None,
        1,
        false,
        false,
        &[],
    )
    .await;

    let id = data::upload(
        &mut client,
        None,
        "Test Model",
        "",
        AssetTypeId::Model,
        None,
        1,
        false,
        false,
        bytes,
    )
    .await
    .unwrap();

    println!("Uploaded new model: {id}");

    client.ensure_token().await.unwrap();
    let result = assets::v1::upload(
        &mut client,
        "test.png",
        "test",
        "",
        AssetType::Decal,
        CreationContext {
            creator: Creator::UserId(authenticted.id.to_string()),
            expected_price: None,
        },
    )
    .await
    .unwrap();

    println!("New asset status: {result:?}");

    let asset_status = assets::v1::status(&mut client, &result.operation_id)
        .await
        .unwrap();

    println!("Asset status: {asset_status:?}");

    let assets = develop::v1::assets(&mut client, &[47433]).await.unwrap();
    println!("Assets: {assets:?}");
}
