use dotenvy_macro::dotenv;
use roblox_api::{Paging, SortOrder, api::users, client::Client};

test_endpoint!(authenticated_details, [users::v1], authenticated_details => |user| {
    assert!(!user.name.is_empty());
});
test_endpoint!(birthdate, [users::v1], birthdate);
test_endpoint!(gender, [users::v1], gender);
test_endpoint!(description, [users::v1], description);
test_endpoint!(user_details, [users::v1], user_details(1) => |user| {
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Roblox");
});
test_endpoint!(users_by_id, [users::v1], users_by_id(&[1u64, 2, 3, 4], false) => |users| {
    assert!(!users.is_empty());
    assert_eq!(users.first().unwrap().id, 1);
});
test_endpoint!(users_by_name, [users::v1], users_by_name(&["Roblox", "test", "word"], false) => |users| {
    assert!(!users.is_empty());
});

#[tokio::test]
async fn validate_display_name_by_id() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let authenticated = users::v1::authenticated_details(&mut client).await.unwrap();
    users::v1::validate_display_name_by_id(&mut client, authenticated.id, "エルデン")
        .await
        .unwrap();
}

#[tokio::test]
async fn user_username_history() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    users::v1::user_username_history(
        &mut client,
        85382088,
        Paging::new(None, Some(100), Some(SortOrder::Ascending)),
    )
    .await
    .unwrap();
}
