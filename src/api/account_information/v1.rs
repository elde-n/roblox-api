use serde::{Deserialize, Serialize};

use crate::{DateTime, Gender, endpoint};

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
pub struct PromotionChannel {
    pub id: u64,
    #[serde(rename = "name")]
    pub promotion_channel: String,
    #[serde(rename = "promotionChannelType")]
    pub kind: Option<u8>,
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
    #[serde(rename = "IsUserDescriptionEnabled")]
    pub is_user_description_enabled: bool,
    #[serde(rename = "isUserEmailOnVerificationEnabled")]
    pub is_user_email_on_verification_enabled: bool,
    #[serde(rename = "isUserAgreementsSignupEnabled")]
    pub is_user_agreements_signup_enabled: bool,
}

endpoint! {
    roblox_badges(id: u64) -> Vec<RobloxBadge> {
        GET "{URL}/users/{id}/roblox-badges";
    }

    birthdate() -> DateTime {
        GET "{URL}/birthdate";
        types {
            BirthdateResponse {
                day("birthDay"): u8,
                month("birthMonth"): u8,
                year("birthYear"): i32,
            }
        }
        map |res: BirthdateResponse| DateTime::from_ymd(res.year, res.month, res.day)
    }

    description() -> String {
        GET "{URL}/description";
        types {
            DescriptionResponse {
                value("description"): String,
            }
        }
        map |res: DescriptionResponse| res.value
    }

    gender() -> Gender {
        GET "{URL}/gender";
        types {
            GenderResponse {
                value("gender"): u8,
            }
        }
        map |res: GenderResponse| Gender::from_repr(res.value).expect("failed to parse gender")
    }

    metadata() -> Metadata {
        GET "{URL}/metadata";
    }

    promotion_channels() -> Vec<PromotionChannel> {
        GET "{URL}/promotion-channels";
    }

    star_code_affiliate() -> u64 {
        GET "{URL}/star-code-affiliates";
        types {
            StarCodeAffiliateResponse {
                id("userId"): u64,
            }
        }
        map |res: StarCodeAffiliateResponse| res.id
    }
}
