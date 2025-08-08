use reqwest::header::{self, HeaderValue};

use crate::{AssetTypeId, Error, client::Client};

pub const URL: &str = "https://data.roblox.com/data";

//"https://data.roblox.com/ide/publish/UploadNewMesh" ?

// perhaps we should have a "update" function too that doesn't take in all these parameters
// apparently decals are also supported, i couldn't get it to work though
//
/// `id` can be set to None, or Some(0) to upload a new asset, using an existing `id` will overwrite the old asset
/// on success RETURNS the new asset id
pub async fn upload(
    client: &mut Client,
    id: Option<u64>,
    name: &str,
    description: &str,
    asset_type: AssetTypeId,
    group_id: Option<u64>,
    genre: u8,
    is_public: bool,
    allow_comments: bool,
    bytes: &[u8],
) -> Result<u64, Error> {
    let id = id.unwrap_or(0);
    let genre_type_id = genre;

    let mut url = format!("{URL}/upload.ashx?assetId={id}");
    if let AssetTypeId::Model = asset_type {
        url.push_str("&type=Model");
    } else if let AssetTypeId::Place = asset_type {
        url.push_str("&type=Place");
    } else {
        let asset_type_id = asset_type as u8;
        url.push_str(&format!("&assetTypeId={asset_type_id}"));
    }

    if let Some(group_id) = group_id {
        url.push_str(&format!("&groupId={group_id}"));
    }

    let mut headers = client.requestor.default_headers.clone();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_str("application/json").unwrap(),
    );

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/octect-stream").unwrap(),
    );

    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_str("Roblox/WinInet").unwrap(),
    );

    let result = client
        .requestor
        .client
        .post(url)
        .query(&[
            ("name", name),
            ("description", description),
            ("genreTypeId", &genre_type_id.to_string()),
            ("isPublic", &is_public.to_string()),
            ("allowComments", &allow_comments.to_string()),
        ])
        .headers(headers)
        .body(bytes.to_owned())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    let id: u64 = response.text().await.unwrap().parse().unwrap();
    Ok(id)
}
