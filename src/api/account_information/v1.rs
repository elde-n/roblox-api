use serde::{Deserialize, Serialize};

use crate::{ApiError, DateTime, Error, Gender, endpoint};

pub const URL: &str = "https://accountinformation.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RobloxBadge {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PromotionChannels {
    pub promotion_channels_visibility_privacy: String,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "isAllowedNotificationsEndpointDisabled")]
    pub is_allowed_notifications_endpoint_disabled: bool,
    #[serde(rename = "isAccountSettingsPolicyEnabled")]
    pub is_account_settings_policy_enabled: bool,
    #[serde(rename = "isPhoneNumberEnabled")]
    pub is_phone_number_enabled: bool,
    #[serde(rename = "MaxUserDescriptionLength")]
    pub max_user_description_length: u64,
    pub is_user_description_enabled: bool,
    #[serde(default, rename = "isUserEmailOnVerificationEnabled")]
    pub is_user_email_on_verification_enabled: bool,
    #[serde(default, rename = "isUserAgreementsSignupEnabled")]
    pub is_user_agreements_signup_enabled: bool,
    #[serde(default)]
    pub is_user_block_endpoints_updated: bool,
    #[serde(default)]
    pub should_use_persona_for_id_verification: bool,
    #[serde(default)]
    pub should_display_session_management: bool,
    #[serde(default)]
    pub is_password_required_for_aging_down: bool,
}

#[endpoint(GET, "{URL}/users/{id}/roblox-badges")]
async fn roblox_badges(id: u64) -> Vec<RobloxBadge> {}

#[endpoint(GET, "{URL}/birthdate")]
async fn birthdate() -> DateTime {
    #[derive(Serialize, Deserialize)]
    struct BirthdateResponse {
        #[serde(rename = "birthDay")]
        pub day: u8,
        #[serde(rename = "birthMonth")]
        pub month: u8,
        #[serde(rename = "birthYear")]
        pub year: i32,
    }
    try_map! { |res: BirthdateResponse| DateTime::from_ymd(res.year, res.month, res.day) }
}

#[endpoint(GET, "{URL}/description")]
async fn description() -> String {
    #[derive(Serialize, Deserialize)]
    struct DescriptionResponse {
        #[serde(rename = "description")]
        pub value: String,
    }
    map! { |res: DescriptionResponse| res.value }
}

#[endpoint(GET, "{URL}/gender")]
async fn gender() -> Gender {
    #[derive(Serialize, Deserialize)]
    struct GenderResponse {
        #[serde(rename = "gender")]
        pub value: u8,
    }
    try_map! { |res: GenderResponse| Gender::from_repr(res.value).ok_or(Error::ApiError(ApiError::InvalidGender)) }
}

#[endpoint(GET, "{URL}/metadata")]
async fn metadata() -> Metadata {}

#[endpoint(GET, "{URL}/promotion-channels")]
async fn promotion_channels() -> PromotionChannels {}

#[endpoint(GET, "{URL}/star-code-affiliates")]
async fn star_code_affiliate() -> u64 {
    #[derive(Serialize, Deserialize)]
    struct StarCodeAffiliateResponse {
        #[serde(rename = "userId")]
        pub id: u64,
    }
    map! { |res: StarCodeAffiliateResponse| res.id }
}
