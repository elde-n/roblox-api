use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{DateTime, endpoint};

pub const URL: &str = "https://clientsettings.roblox.com/v2";
pub const CDN_URL: &str = "https://clientsettingscdn.roblox.com/v2";

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum BinaryType {
    MacPlayer,
    MacStudio,
    WindowsPlayer,
    WindowsStudio,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClientVersionResponse {
    pub version: String,
    pub client_version_upload: String,
    pub bootstrapper_version: String,
    pub next_client_version: Option<String>,
    pub next_client_version_upload: Option<String>,
    pub force_install: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompressionDictionary {
    pub sha256: String,
    pub created_at: DateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OtaInfo {
    pub name: String,
    pub version: String,
    pub download_url: String,
    pub is_standalone: Option<bool>,
    pub asset_id: String,
    pub asset_version: String,
    pub max_app_version: String,
    pub tryout_name: Option<String>,
    #[serde(rename = "localAssetURI")]
    pub local_asset_uri: String,
    pub app_storage_reset_id: String,
    pub is_forced_update: bool,
    pub is_development_config: bool,
    pub assets_manifest: HashMap<String, String>,
    pub version_v2: u64,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelAssignmentType {
    None = 0,
    PerMille = 1,
    BoundToPrivateChannel = 2,
    BoundToPublicChannel = 3,
    OptedInToBetaProgramWithPrivateChannel = 4,
    OptedInToBetaProgramWithPublicChannel = 5,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BetaProgramInfo {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserChannel {
    #[serde(rename = "channelName")]
    pub name: String,
    #[serde(rename = "channelAssignmentType")]
    pub kind: Option<ChannelAssignmentType>,
    pub token: Option<String>,
    pub program: Option<BetaProgramInfo>,
    pub is_flag_only: Option<bool>,
}

impl std::fmt::Display for BinaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn path_segment(name: &str, value: Option<&str>) -> String {
    value
        .map(|value| format!("/{name}/{value}"))
        .unwrap_or_default()
}

/// Get the client version information for a specific binary
///
/// Private channels require a `Roblox-Channel-Token` from [`user_channel`].
#[endpoint(GET, "{URL}/client-version/{binary_type}{channel}")]
async fn client_version(
    binary_type: BinaryType,
    channel_name: Option<&str>,
    channel_token: Option<&str>,
) -> ClientVersionResponse {
    let channel = path_segment("channel", channel_name);
    let channel_token = channel_token.unwrap_or_default();

    headers! {
        "Roblox-Channel-Token" => channel_token,
    }
}

/// Get the over-the-air update information for a specific binary and version
#[endpoint(GET, "{URL}/ota-version/{binary_type}")]
async fn ota_version(
    binary_type: BinaryType,
    version: &str,
    channel: Option<&str>,
    name: Option<&str>,
    tag: Option<&str>,
) -> Vec<OtaInfo> {
    let channel = channel.unwrap_or_default();
    let name = name.unwrap_or_default();
    let tag = tag.unwrap_or_default();

    query! {
        "version" => version,
        "channel" => channel,
        "name" => name,
        "tag" => tag,
    }
}

/// Get the channel assigned to the authenticated user for a specific binary
#[endpoint(GET, "{URL}/user-channel")]
async fn user_channel(binary_type: BinaryType) -> UserChannel {
    let binary_type = binary_type.to_string();

    query! {
        "binaryType" => &binary_type,
    }
}

/// Lists all known compression dictionaries
#[endpoint(GET, "{URL}/compression-dictionaries")]
async fn compression_dictionaries() -> Vec<CompressionDictionary> {
    #[derive(Serialize, Deserialize)]
    struct Response {
        pub dictionaries: Vec<CompressionDictionary>,
    }

    map! { |r: Response| r.dictionaries }
}

/// Returns the specified compression dictionary as bytes
#[endpoint(GET, "{URL}/compression-dictionaries/{hash}")]
async fn compression_dictionary(hash: &str) -> Vec<u8> {
    raw_bytes!();
}

/// Get the client settings for a specific application
#[endpoint(GET, "{URL}/settings/application/{application}{bucket}")]
async fn settings(application: &str, bucket: Option<&str>) -> HashMap<String, String> {
    let bucket = path_segment("bucket", bucket);

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        application_settings: HashMap<String, String>,
    }

    map! { |r: Response| r.application_settings }
}

/// Get the zstd-compressed client settings for a specific application
#[endpoint(
    GET,
    "{CDN_URL}/settings-compressed/application/{application}{bucket}/{hash}.dcz"
)]
async fn settings_compressed(application: &str, hash: &str, bucket: Option<&str>) -> Vec<u8> {
    let bucket = path_segment("bucket", bucket);
    raw_bytes!();
}
