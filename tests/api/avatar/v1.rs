use dotenvy_macro::dotenv;
use roblox_api::{
    Paging,
    api::avatar::{
        self,
        v1::{AvatarScales, AvatarType, BodyColors},
    },
    client::Client,
};
const USER_ID: u64 = 3139503587;

test_endpoint!(user_avatar, [avatar::v1], user_avatar(1));
test_endpoint!(
    user_currently_wearing,
    [avatar::v1],
    user_currently_wearing(1)
);
test_endpoint!(
    set_wearing_assets,
    [avatar::v1],
    avatar_set_wearing_assets(vec![1u64])
);
test_endpoint!(
    avatar_set_type,
    [avatar::v1],
    avatar_set_type(AvatarType::R6)
);
test_endpoint!(
    avatar_set_body_colors,
    [avatar::v1],
    avatar_set_body_colors(BodyColors {
        head: 125,
        torso: 125,
        right_arm: 125,
        left_arm: 125,
        right_leg: 125,
        left_leg: 125,
    })
);
test_endpoint!(
    avatar_set_scales,
    [avatar::v1],
    avatar_set_scales(AvatarScales {
        height: 1.0,
        width: 1.0,
        head: 1.0,
        depth: 1.0,
        proportion: 1.0,
        body_type: 1.0,
    })
);
test_endpoint!(
    user_outfits,
    [avatar::v1],
    user_outfits(1, Paging::default(), None)
);
test_endpoint!(remove_outfit, [avatar::v1], remove_outfit(u64::MAX));

#[tokio::test]
async fn outfit_details() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let result = avatar::v1::user_outfits(&mut client, USER_ID, Paging::default(), None)
        .await
        .unwrap();
    let outfit = result.outfits.first().unwrap();
    let details = avatar::v1::outfit_details(&mut client, outfit.id)
        .await
        .unwrap();
    assert_eq!(outfit.id, details.id);
}
