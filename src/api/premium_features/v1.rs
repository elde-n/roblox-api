use crate::endpoint;

pub const URL: &str = "https://premiumfeatures.roblox.com/v1";

endpoint! {
    is_premium(id: u64) -> bool {
        GET "{URL}/users/{id}/validate-membership";
    }
}
