use serde::Deserialize;

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://games.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceDetails {
    #[serde(rename = "placeId")]
    pub id: u64,
    pub name: String,
    pub description: String,

    pub source_name: String,
    pub source_description: String,

    pub universe_id: u64,
    pub universe_root_place_id: u64,

    pub url: String,
    pub image_token: String,
    pub reason_prohibited: String,

    // cast to Creator
    pub builder: String,
    pub builder_id: u64,

    pub price: u64,

    pub is_playable: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PrivateServerInfoGameRootPlace {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfoGame {
    pub id: u64,
    pub name: String,
    pub root_place: PrivateServerInfoGameRootPlace,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfoSubscription {
    pub price: u64,

    pub active: bool,
    pub expired: bool,
    pub can_renew: bool,
    pub has_price_changed: bool,
    pub has_recurring_profile: bool,
    pub has_insufficient_funds: bool,

    pub expiration_date: DateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfoPermissions {
    // TODO: find out what it holds
    //pub users: Vec<>

    // Assuming u64, might be String
    pub enemy_clan_id: Option<u64>,

    pub clan_allowed: bool,
    pub friends_allowed: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PrivateServerInfoVoiceSettings {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfo {
    pub id: u64,
    pub name: String,

    pub active: bool,

    #[serde(rename = "link")]
    pub link_url: String,
    pub join_code: String,

    pub game: PrivateServerInfoGame,
    pub subscription: PrivateServerInfoSubscription,
    pub permissions: PrivateServerInfoPermissions,
    pub voice_settings: PrivateServerInfoVoiceSettings,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerOwner {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServer {
    pub name: String,
    #[serde(rename = "vipServerId")]
    pub id: u64,
    pub max_players: u16,

    pub owner: PrivateServerOwner,

    //pub players: Vec<()>, // Seems to be an empty vec?
    pub player_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PrivateServersResponse {
    #[serde(rename = "data")]
    pub servers: Vec<PrivateServer>,
    #[serde(rename = "gameJoinRestricted")]
    pub join_restricted: bool,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "id")]
    pub job_id: String,

    pub playing: u16,
    pub max_players: u16,
    pub fps: f32,
    pub ping: u16,

    //pub players: Vec<()>, // Seems to be an empty vec?
    pub player_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ServersResponse {
    #[serde(rename = "data")]
    pub servers: Vec<Server>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UniverseVotes {
    pub id: u64,
    #[serde(rename = "upVotes")]
    pub likes: u32,
    #[serde(rename = "downVotes")]
    pub dislikes: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UniverseGamepass {
    pub id: u64,
    pub name: String,
    pub display_name: String,

    pub price: Option<u64>,
    pub product_id: Option<u64>,

    #[serde(rename = "isOwned")]
    pub owned: bool,

    pub seller_id: Option<u64>,
    pub seller_name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UniverseGamepassesResponse {
    #[serde(rename = "data")]
    pub servers: Vec<UniverseGamepass>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

pub async fn batch_place_details(
    client: &mut Client,
    ids: &[u64],
) -> Result<Vec<PlaceDetails>, Error> {
    let ids = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/multiget-place-details"))
        .query(&[("placeIds", ids)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<Vec<PlaceDetails>>(response)
        .await
}

/// Set server_kind to 0, if you want a valid response
pub async fn servers(
    client: &mut Client,
    id: u64,
    server_kind: u8,
    exclude_full_games: bool,
    paging: Paging<'_>,
) -> Result<ServersResponse, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/{id}/servers/{server_kind}"))
        .query(&[
            ("excludeFullGames", exclude_full_games.to_string()),
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<ServersResponse>(response)
        .await
}

pub async fn private_servers(
    client: &mut Client,
    id: u64,
    exclude_friend_servers: bool,
    paging: Paging<'_>,
) -> Result<PrivateServersResponse, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/{id}/private-servers"))
        .query(&[
            ("excludeFriendServers", exclude_friend_servers.to_string()),
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PrivateServersResponse>(response)
        .await
}

pub async fn private_server_info(client: &mut Client, id: u64) -> Result<PrivateServerInfo, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/vip-servers/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<PrivateServerInfo>(response)
        .await
}

pub async fn universe_favorite_count(client: &mut Client, id: u64) -> Result<u64, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/{id}/favorites/count"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "favoritesCount")]
        favorites: u64,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .favorites)
}

pub async fn universe_votes(client: &mut Client, ids: &[u64]) -> Result<Vec<UniverseVotes>, Error> {
    let ids = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/votes"))
        .query(&[("universeIds", ids)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        votes: Vec<UniverseVotes>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .votes)
}

pub async fn universe_gamepasses(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<UniverseGamepassesResponse, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/{id}/game-passes"))
        .query(&[
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<UniverseGamepassesResponse>(response)
        .await
}
