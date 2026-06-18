use dotenvy_macro::dotenv;
use roblox_api::{
    ApiError, Error, Paging,
    api::{groups, users},
    client::Client,
};

const BHOP_GROUP_ID: u64 = 6980477;
const ROBLOX_GROUP_ID: u64 = 7;
const ROBLOX_GROUP_GUEST_ROLE_ID: u64 = 260;

test_endpoint!(information, [groups::v1], information(BHOP_GROUP_ID));
test_endpoint!(membership, [groups::v1], membership(BHOP_GROUP_ID, false) => |m| {
    assert_eq!(m.id, BHOP_GROUP_ID);
});
test_endpoint!(name_history, [groups::v1], name_history(BHOP_GROUP_ID));
test_endpoint!(pending_join_requests, [groups::v1], pending_join_requests);
test_endpoint!(roles, [groups::v1], roles(BHOP_GROUP_ID));
test_endpoint!(user_roles, [groups::v1], user_roles(BHOP_GROUP_ID));
test_endpoint!(roleset_permissions, [groups::v1], roleset_permissions(ROBLOX_GROUP_ID, ROBLOX_GROUP_GUEST_ROLE_ID) => |permissions| {
    assert_eq!(permissions.id, ROBLOX_GROUP_ID);
    assert_eq!(permissions.role.id, ROBLOX_GROUP_GUEST_ROLE_ID);
    assert_eq!(permissions.role.rank, 0);
});
test_endpoint!(users, [groups::v1], users(BHOP_GROUP_ID, Paging::default()));
test_endpoint!(
    wall_posts,
    [groups::v1],
    wall_posts(BHOP_GROUP_ID, Paging::default())
);

#[tokio::test]
async fn join() -> Result<(), Error> {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result = groups::v1::join(&mut client, ROBLOX_GROUP_ID).await;
    if let Err(Error::ApiError(ApiError::AlreadyInGroup)) = result {
        Ok(())
    } else {
        result
    }
}

#[tokio::test]
async fn join_request() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let details = users::v1::authenticated_details(&mut client).await.unwrap();
    groups::v1::remove_join_request(&mut client, BHOP_GROUP_ID, details.id)
        .await
        .unwrap();
    groups::v1::join(&mut client, BHOP_GROUP_ID).await.unwrap();
}
