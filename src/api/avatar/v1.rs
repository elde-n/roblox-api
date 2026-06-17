use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, FromRepr};

use crate::{Paging, endpoint};

pub const URL: &str = "https://avatar.roblox.com/v1";

pub type ColorId = u8;

#[repr(u8)]
#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumString, FromRepr,
)]
pub enum AvatarType {
    R6 = 1,
    R15 = 2,
}

#[repr(u8)]
#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Display, EnumString, FromRepr,
)]
pub enum MorphAvatarType {
    MorphR6 = 1,
    MorphR15 = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarScales {
    pub height: f32,
    pub width: f32,
    pub head: f32,
    pub depth: f32,
    pub proportion: f32,
    pub body_type: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BodyColors {
    #[serde(rename = "headColorId")]
    pub head: ColorId,
    #[serde(rename = "torsoColorId")]
    pub torso: ColorId,
    #[serde(rename = "rightArmColorId")]
    pub right_arm: ColorId,
    #[serde(rename = "leftArmColorId")]
    pub left_arm: ColorId,
    #[serde(rename = "rightLegColorId")]
    pub right_leg: ColorId,
    #[serde(rename = "leftLegColorId")]
    pub left_leg: ColorId,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AssetType {
    pub id: u8,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AssetMeta {
    pub version: u8,

    pub order: u16,
    pub puffiness: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: u64,
    pub name: String,
    #[serde(rename = "assetType")]
    pub kind: AssetType,
    pub current_version_id: u64,
    pub meta: Option<AssetMeta>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    #[serde(rename = "assetId")]
    pub id: u64,
    #[serde(rename = "assetName")]
    pub name: String,
    pub position: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Outfit {
    pub id: u64,
    pub name: String,
    pub is_editable: bool,
    pub outfit_type: Option<()>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarResponse {
    #[serde(rename = "playerAvatarType")]
    pub kind: AvatarType,
    pub assets: Vec<Asset>,
    pub scales: AvatarScales,
    pub body_colors: BodyColors,
    pub default_pants_applied: bool,
    pub default_shirt_applied: bool,
    pub emotes: Vec<Emote>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OutfitsResponse {
    #[serde(rename = "data")]
    pub outfits: Vec<Outfit>,
    pub total: u64,
    pub filtered_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OutfitDetails {
    pub id: u64,
    pub name: String,
    pub universe_id: u64,
    pub assets: Vec<Asset>,
    pub body_colors: BodyColors,
    #[serde(rename = "scale")]
    pub scales: AvatarScales,
    #[serde(rename = "playerAvatarType")]
    pub avatar_type: AvatarType,
    pub outfit_type: String,
    pub is_editable: bool,
    pub moderation_status: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UniverseAvatarSettings {
    #[serde(rename = "gameAvatarType")]
    pub avatar_type: MorphAvatarType,
    pub avatar_body_type: String,
    pub avatar_collision_type: String,
    pub joint_positioning_type: String,

    pub avatar_min_scales: AvatarScales,
    pub avatar_max_scales: AvatarScales,
    pub avatar_asset_overrides: Vec<Option<()>>,

    pub message: String,
    pub moderation_status: Option<String>,

    pub allow_custom_animations: String,
}

endpoint! {
    /// Returns details about a specified user's avatar
    user_avatar(id: u64) -> AvatarResponse {
        GET "{URL}/users/{id}/avatar";
    }

    /// Gets a list of asset ids that the user is currently wearing
    user_currently_wearing(id: u64) -> Vec<u64> {
        GET "{URL}/users/{id}/currently-wearing";
        types {
            Response {
                ids("assetIds"): Vec<u64>,
            }
        }
        map |r: Response| r.ids
    }

    /// Sets the avatar's current assets to the list - Flagged as obsolete, does not support layered clothing meta params.
    ///
    /// Warning: Deprecated
    /// Only allows items that you own, are not expired, and are wearable asset types.
    /// Any assets being worn before this method is called are automatically removed.
    avatar_set_wearing_assets(assets: Vec<u64>) -> bool {
        POST "{URL}/avatar/set-wearing-assets";
        types {
            Request<'a> {
                asset_ids("assetIds"): &'a [u64],
            }
            Response {
                success: bool,
            }
        }
        body_serialize {
            &Request { asset_ids: &assets }
        }
        map |r: Response| r.success
    }

    /// Sets the authenticated user's player avatar type (e.g. R6 or R15).
    avatar_set_type(kind: AvatarType) -> bool {
        POST "{URL}/avatar/set-player-avatar-type";
        types {
            Request {
                avatar_type("playerAvatarType"): AvatarType,
            }
            Response {
                success: bool,
            }
        }
        body_serialize {
            &Request { avatar_type: kind }
        }
        map |r: Response| r.success
    }

    /// Sets the authenticated user's body colors.
    avatar_set_body_colors(colors: BodyColors) -> bool {
        POST "{URL}/avatar/set-body-colors";
        types {
            Response {
                success: bool,
            }
        }
        body_serialize {
            &colors
        }
        map |r: Response| r.success
    }

    /// Sets the authenticated user's body scales.
    avatar_set_scales(scales: AvatarScales) -> bool {
        POST "{URL}/avatar/set-scales";
        types {
            Response {
                success: bool,
            }
        }
        body_serialize {
            &scales
        }
        map |r: Response| r.success
    }

    /// Deprecated, use v2. Gets a list of outfits for the specified user.
    user_outfits(
        id: u64, paging: Paging<'_>, is_editable: Option<bool>
    ) -> OutfitsResponse {
        GET "{URL}/users/{id}/outfits";
        prelude {
            let limit = paging.limit.unwrap_or(25).to_string();
            let cursor = paging.cursor.unwrap_or("1");
            let is_editable = match is_editable {
                Some(editable) => editable.to_string(),
                None => String::new(),
            };
        }
        query {
            "page" => cursor,
            "itemsPerPage" => &limit,
            "isEditable" => &is_editable,
        }
    }

    /// Gets details about the contents of an outfit.
    outfit_details(id: u64) -> OutfitDetails {
        GET "{URL}/outfits/{id}/details";
    }

    /// Deletes the outfit.
    remove_outfit(id: u64) -> bool {
        POST "{URL}/outfits/{id}/delete";
        types {
            Response {
                success: bool,
            }
        }
        map |r: Response| r.success
    }

    /// The server will call this on game server start to request general information about the universe.
    /// This is version 1.1, which returns an entry from the UniverseAvatarType enum.
    /// During mixed mode this may return unreliable results
    universe_avatar_settings(id: u64) -> UniverseAvatarSettings {
        GET "{URL}/users/{id}/avatar";
    }
}
