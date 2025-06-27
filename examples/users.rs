use roblox_api::{
    Paging, SortOrder,
    api::{
        inventory::{self, v1::ItemType},
        users,
    },
    client::Client,
};

#[tokio::main]
async fn main() {
    let mut client = Client::default();
    let details = users::v1::user_details(&mut client, 1).await.unwrap();
    println!("details: {:?}", details);

    let name_history = users::v1::user_username_history(
        &mut client,
        85382088,
        Paging::new(None, Some(100), Some(SortOrder::Ascending)),
    )
    .await
    .unwrap();

    println!("username history: {name_history:?}");

    let users = users::v1::users_by_id(&mut client, &[1, 2, 3, 4], false)
        .await
        .unwrap();

    println!("Users by id: {users:?}");

    let users = users::v1::users_by_name(&mut client, &["Roblox", "test", "word"], false)
        .await
        .unwrap();

    println!("Users by id: {users:?}");

    let owned_assets = inventory::v1::user_owned_assets(
        &mut client,
        3139503587,
        4391384843,
        ItemType::Asset,
        Paging::default(),
    )
    .await
    .unwrap();

    println!("Owned assets: {:?}", owned_assets);

    let owned_collectibles =
        inventory::v1::user_owned_collectibles(&mut client, 3139503587, None, Paging::default())
            .await
            .unwrap();

    println!("Owned collectibles: {:?}", owned_collectibles);
}
