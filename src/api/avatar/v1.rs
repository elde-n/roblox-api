use reqwest::Method;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{Error, Paging, client::Client};

pub const URL: &str = "https://avatar.roblox.com/v1";

pub type ColorId = u8;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AvatarType {
    R6 = 1,
    R15 = 2,
}

impl std::fmt::Display for AvatarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AvatarType::R6 => "R6",
                AvatarType::R15 => "R15",
            }
        )
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    // AssetTypeId, but repr lol
    pub id: u8,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AssetMeta {
    pub version: u8,

    pub order: u16,
    pub puffiness: f32,
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
    pub outfit_type: String, // TODO: change to enum
    pub is_editable: bool,
    pub moderation_status: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UniverseAvatarSettings {
    #[serde(rename = "gameAvatarType")]
    pub avatar_type: MorphAvatarType,
    pub avatar_body_type: String,       // TDOO: change to Enum
    pub avatar_collision_type: String,  // TDOO: change to Enum
    pub joint_positioning_type: String, // TDOO: change to Enum

    pub avatar_min_scales: AvatarScales,
    pub avatar_max_scales: AvatarScales,
    pub avatar_asset_overrides: Vec<Option<()>>,

    pub message: String,
    pub moderation_status: Option<String>,

    pub allow_custom_animations: String, // TODO: cast to bool
}

async fn generic_request<'a, R: Serialize, T: DeserializeOwned>(
    client: &mut Client,
    method: Method,
    path: &str,
    request: Option<&'a R>,
    query: Option<&'a [(&'a str, &'a str)]>,
) -> Result<T, Error> {
    let mut builder = client
        .requestor
        .client
        .request(method, format!("{URL}/{path}"))
        .headers(client.requestor.default_headers.clone());

    if let Some(request) = request {
        builder = builder.json(&request);
    }

    if let Some(query) = query {
        builder = builder.query(&query);
    }

    let response = client.validate_response(builder.send().await).await?;
    client.requestor.parse_json::<T>(response).await
}

/// Returns details about a specified user's avatar
pub async fn user_avatar(client: &mut Client, id: u64) -> Result<AvatarResponse, Error> {
    generic_request::<(), AvatarResponse>(
        client,
        Method::GET,
        &format!("/users/{id}/avatar"),
        None,
        None,
    )
    .await
}

/// Gets a list of asset ids that the user is currently wearing
pub async fn user_currently_wearing(client: &mut Client, id: u64) -> Result<Vec<u64>, Error> {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "assetIds")]
        ids: Vec<u64>,
    }

    Ok(generic_request::<(), Response>(
        client,
        Method::GET,
        &format!("/users/{id}/currently-wearing"),
        None,
        None,
    )
    .await?
    .ids)
}

/// Sets the avatar's current assets to the list - Flagged as obsolete, does not support layered clothing meta params.
///
/// Warning: Deprecated
/// Only allows items that you own, are not expired, and are wearable asset types.
/// Any assets being worn before this method is called are automatically removed.
pub async fn avatar_set_wearing_assets(
    client: &mut Client,
    assets: Vec<u64>,
) -> Result<bool, Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Request<'a> {
        asset_ids: &'a [u64],
    }

    #[derive(Deserialize)]
    pub struct Response {
        pub success: bool,
    }

    Ok(generic_request::<Request, Response>(
        client,
        Method::POST,
        &format!("/avatar/set-wearing-assets"),
        Some(&Request { asset_ids: &assets }),
        None,
    )
    .await?
    .success)
}

/// Sets the authenticated user's player avatar type (e.g. R6 or R15).
pub async fn avatar_set_type(client: &mut Client, kind: AvatarType) -> Result<bool, Error> {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename = "playerAvatarType")]
        avatar_type: AvatarType,
    }

    #[derive(Deserialize)]
    struct Response {
        success: bool,
    }

    Ok(generic_request::<Request, Response>(
        client,
        Method::POST,
        &format!("/avatar/set-player-avatar-type"),
        Some(&Request { avatar_type: kind }),
        None,
    )
    .await?
    .success)
}

/// Sets the authenticated user's body colors.
pub async fn avatar_set_body_colors(
    client: &mut Client,
    colors: BodyColors,
) -> Result<bool, Error> {
    #[derive(Deserialize)]
    struct Response {
        success: bool,
    }

    Ok(generic_request::<BodyColors, Response>(
        client,
        Method::POST,
        &format!("/avatar/set-body-colors"),
        Some(&colors),
        None,
    )
    .await?
    .success)
}

/// Sets the authenticated user's body colors.
pub async fn avatar_set_scales(client: &mut Client, scales: AvatarScales) -> Result<bool, Error> {
    #[derive(Deserialize)]
    struct Response {
        success: bool,
    }

    Ok(generic_request::<AvatarScales, Response>(
        client,
        Method::POST,
        &format!("/avatar/set-scales"),
        Some(&scales),
        None,
    )
    .await?
    .success)
}

/// Deprecated, user v2. Gets a list of outfits for the specified user.
pub async fn user_outfits(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
    is_editable: Option<bool>,
    //outfit_type: OutfitType, all seem to be null
) -> Result<OutfitsResponse, Error> {
    let limit = paging.limit.unwrap_or(25).to_string();
    let cursor = paging.cursor.unwrap_or("1");
    let is_editable = match is_editable {
        Some(editable) => editable.to_string(),
        None => "".to_string(),
    };

    generic_request::<(), OutfitsResponse>(
        client,
        Method::GET,
        &format!("/users/{id}/outfits"),
        None,
        Some(&[
            ("page", cursor),
            ("itemsPerPage", &limit),
            ("isEditable", &is_editable),
        ]),
    )
    .await
}

/// Gets details about the contents of an outfit.
pub async fn outfit_details(client: &mut Client, id: u64) -> Result<OutfitDetails, Error> {
    generic_request::<(), OutfitDetails>(
        client,
        Method::GET,
        &format!("/outfits/{id}/details"),
        None,
        None,
    )
    .await
}

/// Deletes the outfit.
pub async fn remove_outfit(client: &mut Client, id: u64) -> Result<bool, Error> {
    #[derive(Deserialize)]
    struct Response {
        success: bool,
    }

    Ok(generic_request::<(), Response>(
        client,
        Method::POST,
        &format!("/outfits/{id}/delete"),
        None,
        None,
    )
    .await?
    .success)
}

/// The server will call this on game server start to request general information about the universe.
/// This is version 1.1, which returns an entry from the UniverseAvatarType enum.
/// During mixed mode this may return unreliable results
pub async fn universe_avatar_settings(
    client: &mut Client,
    id: u64,
) -> Result<UniverseAvatarSettings, Error> {
    generic_request::<(), UniverseAvatarSettings>(
        client,
        Method::GET,
        &format!("/users/{id}/avatar"),
        None,
        None,
    )
    .await
}
