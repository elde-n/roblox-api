use crate::endpoint;

pub const URL: &str = "https://apis.roblox.com/hba-service/v1";

#[endpoint(GET, "{URL}/getservernonce")]
async fn server_nonce() -> String {}
