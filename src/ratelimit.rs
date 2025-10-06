use crate::client::ClientRequestor;

pub(crate) const RATELIMIT_LIMIT_HEADER: &str = "x-ratelimit-limit";
pub(crate) const RATELIMIT_RESET_HEADER: &str = "x-ratelimit-reset";
pub(crate) const RATELIMIT_REMAINING_HEADER: &str = "x-ratelimit-remaining";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Ratelimit {
    pub remaining: u32,
    pub reset_in_seconds: u32,
    pub windows: Vec<(u32, u32)>, // Amount:seconds
}

impl ClientRequestor {
    pub(crate) async fn ratelimits(&self) -> Option<Ratelimit> {
        self.ratelimit.clone()
    }
}
