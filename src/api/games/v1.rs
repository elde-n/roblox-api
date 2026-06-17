use serde::{Deserialize, Serialize};

use crate::{DateTime, Paging, endpoint};

pub const URL: &str = "https://games.roblox.com/v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrivateServerInfoGameRootPlace {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfoGame {
    pub id: u64,
    pub name: String,
    pub root_place: PrivateServerInfoGameRootPlace,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerInfoPermissions {
    // TODO: find out what it holds
    //pub users: Vec<>

    // Assuming u64, might be String
    pub enemy_clan_id: Option<u64>,

    pub clan_allowed: bool,
    pub friends_allowed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PrivateServerInfoVoiceSettings {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServerOwner {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "id")]
    pub job_id: String,

    pub playing: u16,
    pub max_players: u16,
    pub fps: f32,
    pub ping: Option<u16>,

    //pub players: Vec<()>, // Seems to be an empty vec?
    pub player_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServersResponse {
    #[serde(rename = "data")]
    pub servers: Vec<Server>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UniverseVotes {
    pub id: u64,
    #[serde(rename = "upVotes")]
    pub likes: u32,
    #[serde(rename = "downVotes")]
    pub dislikes: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UniverseGamepassesResponse {
    #[serde(rename = "data")]
    pub gamepasses: Vec<UniverseGamepass>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

endpoint! {
    batch_place_details(ids: &[u64]) -> Vec<PlaceDetails> {
        GET "{URL}/games/multiget-place-details";
        prelude {
            let joined_ids = ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        query {
            "placeIds" => &joined_ids,
        }
    }

    /// Set server_kind to 0, if you want a valid response
    servers(id: u64, server_kind: u8, exclude_full_games: bool, paging: Paging<'_>) -> ServersResponse {
        GET "{URL}/games/{id}/servers/{server_kind}";
        paging_query { paging, limit = 10 }
        prelude {
            let exclude_full_games = exclude_full_games.to_string();
        }
        query {
            "excludeFullGames" => &exclude_full_games,
        }
    }

    private_servers(id: u64, exclude_friend_servers: bool, paging: Paging<'_>) -> PrivateServersResponse {
        GET "{URL}/games/{id}/private-servers";
        paging_query { paging, limit = 10 }
        prelude {
            let exclude_friend_servers = exclude_friend_servers.to_string();
        }
        query {
            "excludeFriendServers" => &exclude_friend_servers,
        }
    }

    private_server_info(id: u64) -> PrivateServerInfo {
        GET "{URL}/vip-servers/{id}";
    }

    universe_favorite_count(id: u64) -> u64 {
        GET "{URL}/games/{id}/favorites/count";
        types {
            Response {
                favorites("favoritesCount"): u64,
            }
        }
        map |r: Response| r.favorites
    }

    universe_votes(ids: &[u64]) -> Vec<UniverseVotes> {
        GET "{URL}/games/votes";
        types {
            Response {
                votes("data"): Vec<UniverseVotes>,
            }
        }
        prelude {
            let joined_ids = ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        query {
            "universeIds" => &joined_ids,
        }
        map |r: Response| r.votes
    }

    universe_gamepasses(id: u64, paging: Paging<'_>) -> UniverseGamepassesResponse {
        GET "{URL}/games/{id}/game-passes";
        paging_query { paging, limit = 10 }
    }
}
