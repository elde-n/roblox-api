pub mod api;
pub mod challenge;
pub mod client;
pub mod validation;

use challenge::Challenge;
use chrono::{Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};

// TODO: using this wrapper as I couldn't figure out how to use chronos datetime alone
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DateTime(String);
impl DateTime {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        Self(
            Utc.with_ymd_and_hms(year, month as u32, day as u32, 0, 0, 0)
                .unwrap()
                .to_rfc3339(),
        )
    }

    pub fn day(&self) -> u8 {
        chrono::DateTime::parse_from_rfc3339(&self.0).unwrap().day() as u8
    }

    pub fn month(&self) -> u8 {
        chrono::DateTime::parse_from_rfc3339(&self.0)
            .unwrap()
            .month() as u8
    }

    pub fn year(&self) -> i32 {
        chrono::DateTime::parse_from_rfc3339(&self.0)
            .unwrap()
            .year()
    }
}

impl ToString for DateTime {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    BadResponse,
    ReqwestError(reqwest::Error),
    IoError(std::io::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    Internal,
    BadRequest,
    RequestMissingArgument(String),
    Ratelimited,
    Unknown(u16),
    Unauthorized,
    InvalidBirthdate,
    InvalidDisplayName,
    InvalidGender,
    UserNotFound,
    InvalidUserId,
    PinIsLocked,
    TokenValidation,
    ChallengeRequired(Challenge),
    ChallengeFailed,
    InvalidChallengeId,
    InvalidTwoStepVerificationCode,
    TwoStepVerificationMaintenance,
    Multiple(Vec<ApiError>),
    PermissionError,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum AssetTypeId {
    Image = 1,
    TShirt,
    Audio,
    Mesh,
    Lua,
    Hat = 8,
    Place,
    Model,
    Shirt,
    Pants,
    Decal,
    Head = 17,
    Face,
    Gear,
    Badge = 21,
    Animation = 24,
    Torso = 27,
    RightArm,
    LeftArm,
    LeftLeg,
    RightLeg,
    Package,
    Gamepass = 34,
    Plugin = 38,
    MeshPart = 40,
    HairAccessory,
    FaceAccessory,
    NeckAccessory,
    ShoulderAccessory,
    FrontAccessory,
    BackAccessory,
    WaistAccessory,
    ClimbAnimation,
    DeathAnimation,
    FallAnimation,
    IdleAnimation,
    JumpAnimation,
    RunAnimation,
    SwimAnimation,
    WalkAnimation,
    PoseAnimation,
    EarAccessory,
    EyeAccessory,
    EmoteAnimation = 61,
    Video,
    TShirtAccessory = 64,
    ShirtAccessory,
    PantsAccessory,
    JacketAccessory,
    SweaterAccessory,
    ShortsAccessory,
    LeftShoeAccessory,
    RightShoeAccessory,
    DressSkirtAccessory,
    FontFamily,
    EyebrowAccessory = 76,
    EyelashAccessory,
    MoodAnimation,
    DynamicHead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Paging<'a> {
    pub cursor: Option<&'a str>,
    pub limit: Option<u16>,
    pub order: Option<SortOrder>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
pub enum SortOrder {
    #[default]
    #[serde(rename = "Asc")]
    Ascending,
    #[serde(rename = "Desc")]
    Descending,
}

impl ToString for SortOrder {
    fn to_string(&self) -> String {
        match self {
            SortOrder::Ascending => "Asc".to_string(),
            SortOrder::Descending => "Desc".to_string(),
        }
    }
}

impl Default for Paging<'_> {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: Some(10),
            order: Some(SortOrder::Ascending),
        }
    }
}

impl<'a> Paging<'a> {
    pub fn new(
        cursor: Option<&'a str>,
        limit: Option<u16>,
        order: Option<SortOrder>,
    ) -> Paging<'a> {
        Self {
            cursor,
            limit,
            order,
        }
    }
}
