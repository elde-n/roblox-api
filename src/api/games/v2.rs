use serde::{Deserialize, Serialize};

use crate::{AssetTypeId, DateTime, Error, Paging, client::Client};

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

async fn games_generic(
    client: &mut Client,
    path: &str,
    access_filter: u8,
    paging: Paging<'_>,
) -> Result<GamesResponse, Error> {
    let limit = paging.limit.unwrap_or(10).to_string();
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => cursor.to_string(),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!("{URL}/{path}"))
        .query(&[
            ("accessFilter", access_filter.to_string()),
            ("limit", limit),
            ("sortOrder", sort_order),
            ("cursor", cursor),
        ])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.requestor.validate_response(result).await?;
    client.requestor.parse_json::<GamesResponse>(response).await
}

pub async fn universe_media(
    client: &mut Client,
    id: u64,
    all_experiences: bool,
) -> Result<Vec<UniverseMedia>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/games/{id}/media"))
        .query(&[("fetchAllExperienceRelatedMedia", all_experiences)])
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        media: Vec<UniverseMedia>,
    }

    let response = client.requestor.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .media)
}

/// Apparently this api only works on owned groups, use v2 instead
/// Set `access_filter` to 1, if you want a valid response
pub async fn group_games(
    client: &mut Client,
    id: u64,
    access_filter: u8,
    paging: Paging<'_>,
) -> Result<GamesResponse, Error> {
    games_generic(client, &format!("groups/{id}/games"), access_filter, paging).await
}

/// Set `access_filter` to 1, if you want a valid response
pub async fn group_games_v2(
    client: &mut Client,
    id: u64,
    access_filter: u8,
    paging: Paging<'_>,
) -> Result<GamesResponse, Error> {
    games_generic(
        client,
        &format!("groups/{id}/gamesV2"),
        access_filter,
        paging,
    )
    .await
}

/// Set `access_filter` to 2, if you want a valid response
pub async fn user_games(
    client: &mut Client,
    id: u64,
    access_filter: u8,
    paging: Paging<'_>,
) -> Result<GamesResponse, Error> {
    games_generic(client, &format!("users/{id}/games"), access_filter, paging).await
}

/// Set `access_filter` to 2, if you want a valid response
pub async fn user_favorited_games(
    client: &mut Client,
    id: u64,
    access_filter: u8,
    paging: Paging<'_>,
) -> Result<GamesResponse, Error> {
    games_generic(
        client,
        &format!("users/{id}/favorite/games"),
        access_filter,
        paging,
    )
    .await
}
