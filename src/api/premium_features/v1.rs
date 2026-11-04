use crate::endpoint;

pub const URL: &str = "https://premiumfeatures.roblox.com/v1";

#[endpoint(GET, "{URL}/users/{id}/validate-membership")]
async fn is_premium(id: u64) -> bool {}
