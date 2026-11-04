use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, DateTime, Paging, endpoint};

pub const URL: &str = "https://games.roblox.com/v2";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UniverseMedia {
    pub asset_type: AssetTypeId,
    pub asset_type_id: u8,

    pub approved: bool,
    pub alt_text: Option<String>,

    pub image_id: Option<u64>,

    pub video_id: Option<String>,
    pub video_hash: Option<String>,
    pub video_title: Option<String>,
}

// TODO: use CreatorType instead
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GameCreator {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GameRootPlace {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,

    pub creator: GameCreator,
    pub root_place: GameRootPlace,

    pub created: DateTime,
    pub updated: DateTime,

    pub price: Option<u64>,
    pub place_visits: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GamesResponse {
    #[serde(rename = "data")]
    pub games: Vec<Game>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[endpoint(GET, "{URL}/games/{id}/media")]
async fn universe_media(id: u64, all_experiences: bool) -> Vec<UniverseMedia> {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        pub media: Vec<UniverseMedia>,
    }
    let all_experiences = all_experiences.to_string();

    query! {
        "fetchAllExperienceRelatedMedia" => &all_experiences,
    }
    map! { |r: Response| r.media }
}

/// Apparently this API only works on owned groups, use v2 instead
/// Set `access_filter` to 1, if you want a valid response
#[endpoint(GET, "{URL}/groups/{id}/games")]
async fn group_games(id: u64, access_filter: u8, paging: Paging<'_>) -> GamesResponse {
    paging!(paging, limit = 10);
    let access_filter = access_filter.to_string();

    query! {
        "accessFilter" => &access_filter,
    }
}

/// Set `access_filter` to 1, if you want a valid response
#[endpoint(GET, "{URL}/groups/{id}/gamesV2")]
async fn group_games_v2(id: u64, access_filter: u8, paging: Paging<'_>) -> GamesResponse {
    paging!(paging, limit = 10);
    let access_filter = access_filter.to_string();

    query! {
        "accessFilter" => &access_filter,
    }
}

/// Set `access_filter` to 2, if you want a valid response
#[endpoint(GET, "{URL}/users/{id}/games")]
async fn user_games(id: u64, access_filter: u8, paging: Paging<'_>) -> GamesResponse {
    paging!(paging, limit = 10);
    let access_filter = access_filter.to_string();

    query! {
        "accessFilter" => &access_filter,
    }
}

/// Set `access_filter` to 2, if you want a valid response
#[endpoint(GET, "{URL}/users/{id}/favorite/games")]
async fn user_favorited_games(id: u64, access_filter: u8, paging: Paging<'_>) -> GamesResponse {
    paging!(paging, limit = 10);
    let access_filter = access_filter.to_string();

    query! {
        "accessFilter" => &access_filter,
    }
}
