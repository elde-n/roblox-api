pub mod api;
pub mod challenge;
pub mod client;
pub mod ratelimit;
pub mod validation;

use challenge::Challenge;
use chrono::{Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

// TODO: using this wrapper as I couldn't figure out how to use chronos datetime alone
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    BadJson,
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    #[cfg(feature = "web-socket")]
    ReqwestWebSocketError(reqwest_websocket::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    Internal,
    BadRequest,
    RequestMissingArgument(String),
    Ratelimited,
    Unknown(u16, Option<String>),
    Unauthorized,
    InvalidBirthdate,
    InvalidDisplayName,
    InvalidGender,
    InvalidUser,
    InvalidUserId,
    PinIsLocked,
    TokenValidation,
    CaptchaFailed,
    ChallengeRequired(Challenge),
    ChallengeFailed,
    InvalidChallengeId,
    InvalidTwoStepVerificationCode,
    TwoStepVerificationMaintenance,
    Multiple(Vec<ApiError>),
    PermissionError,
    AccontLocked,
    AccountIssue,
    InvalidCredentials,
    UnverifiedCredentials,
    ExistingLoginSession,
    DefaultLoginRequired,
    VNGAppLoginRequired,
    LuoBuAppLoginRequired,
    SocialNetworkLoginRequired,
    InvalidAssetId,
    InvalidBrowserTrackerId,
    AlreadyInGroup,
    AlreadyInGroupRequests,
    UnsupportedSortOrder,
    InvalidBadge,
    ConversationCreationFailed,
    InvalidConversation,
    ConversationUserAddFailed,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, EnumIter)]
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
    YouTubeVideo,
    Gamepass,
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

#[repr(u8)]
#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub enum Currency {
    #[default]
    Robux = 1,
    Tickets,
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

impl From<serde_json::Error> for Error {
    fn from(_error: serde_json::Error) -> Self {
        Error::BadJson
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

#[cfg(feature = "web-socket")]
impl From<reqwest_websocket::Error> for Error {
    fn from(error: reqwest_websocket::Error) -> Self {
        Error::ReqwestWebSocketError(error)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for AssetTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for AssetTypeId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for kind in AssetTypeId::iter() {
            if kind.to_string().as_str() == value {
                return Ok(kind);
            }
        }

        Err("Failed to convert string to AssetTypeId")
    }
}

impl TryFrom<u8> for AssetTypeId {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for kind in AssetTypeId::iter() {
            if kind as u8 == value {
                return Ok(kind);
            }
        }

        Err("Failed to convert u8 to AssetTypeId")
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortOrder::Ascending => "Asc",
                SortOrder::Descending => "Desc",
            }
        )
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
