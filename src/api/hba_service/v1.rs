use crate::endpoint;

pub const URL: &str = "https://apis.roblox.com/hba-service/v1";

endpoint! {
    server_nonce() -> String {
        GET "{URL}/getservernonce";
    }
}
