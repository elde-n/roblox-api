use roblox_api::{
    api::users,
    client::{Client, Cookie},
};

#[tokio::main]
async fn main() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    client.ensure_token().await.unwrap();

    let authenticted = users::v1::authenticated_details(&mut client).await.unwrap();
    println!("UserDetails: {:?}", authenticted);

    let birthdate = users::v1::birthdate(&mut client).await.unwrap();
    println!("Current birthdate: {:?}", birthdate);

    let gender = users::v1::gender(&mut client).await.unwrap();
    println!("Current gender: {:?}", gender);

    let description = users::v1::description(&mut client).await.unwrap();
    println!("Current description: {:?}", description);

    let validation_result =
        users::v1::validate_display_name_by_id(&mut client, authenticted.id, "エルデン").await;
    println!("Display name validation result: {validation_result:?}");
}
