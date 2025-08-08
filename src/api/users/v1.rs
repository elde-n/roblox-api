use serde::{Deserialize, Serialize};

use crate::{DateTime, Error, Paging, client::Client};

pub const URL: &str = "https://users.roblox.com/v1";

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gender {
    None = 1,
    Male = 2,
    Female = 3,
}

impl Gender {
    fn from_u8(value: u8) -> Self {
        match value {
            2 => Self::Male,
            3 => Self::Female,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ClientDetails {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ClientAppLaunchInfo {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isPremium")]
    pub is_premium: bool,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "ageBracket")]
    pub age_bracket: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UserDetails {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub description: String,
    #[serde(rename = "created")]
    pub created_date: DateTime,
    #[serde(rename = "isBanned")]
    pub is_terminated: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UserById {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UserByName {
    pub id: u64,
    pub name: String,
    #[serde(rename = "requestedUsername")]
    pub requested_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UserByKeyword {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "previousUsernames")]
    pub previous_names: Vec<String>,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct NameHistory {
    pub names: Vec<String>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UserSearchByKeyword {
    #[serde(rename = "data")]
    pub users: Vec<UserByKeyword>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

pub async fn user_details(client: &mut Client, id: u64) -> Result<UserDetails, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/{id}"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<UserDetails>(response).await
}

pub async fn user_username_history(
    client: &mut Client,
    id: u64,
    paging: Paging<'_>,
) -> Result<NameHistory, Error> {
    let limit = paging.limit.unwrap_or(10);
    let sort_order = paging.order.unwrap_or_default().to_string();
    let cursor = match paging.cursor {
        Some(cursor) => format!("&cursor={cursor}"),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!(
            "{URL}/users/{id}/username-history?limit={limit}&sortOrder={sort_order}{cursor}"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    // no they don't have anything else in here
    #[derive(Debug, Deserialize)]
    struct Username {
        name: String,
    }

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        names: Vec<Username>,
        #[serde(rename = "nextPageCursor")]
        next_cursor: Option<String>,
        #[serde(rename = "previousPageCursor")]
        previous_cursor: Option<String>,
    }

    let response = client.validate_response(result).await?;
    let result = client.requestor.parse_json::<Response>(response).await?;

    let names = result.names.iter().map(|x| x.name.clone()).collect();
    Ok(NameHistory {
        names,
        next_cursor: result.next_cursor,
        previous_cursor: result.previous_cursor,
    })
}

pub async fn users_by_id(
    client: &mut Client,
    ids: &[u64],
    exclude_terminated: bool,
) -> Result<Vec<UserById>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "userIds")]
        ids: &'a [u64],
        #[serde(rename = "excludeBannedUsers")]
        exclude_terminated: bool,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/users"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            ids,
            exclude_terminated,
        })
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        users: Vec<UserById>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .users)
}

pub async fn users_by_name(
    client: &mut Client,
    names: &[&str],
    exclude_terminated: bool,
) -> Result<Vec<UserByName>, Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "usernames")]
        names: &'a [&'a str],
        #[serde(rename = "excludeBannedUsers")]
        exclude_terminated: bool,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/usernames/users"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            names,
            exclude_terminated,
        })
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "data")]
        names: Vec<UserByName>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .names)
}

// this api seems to be dysfunctional, try using search::omni instead
pub async fn search_by_keyword(
    client: &mut Client,
    keyword: &str,
    session_id: Option<&str>,
    paging: Paging<'_>,
) -> Result<UserSearchByKeyword, Error> {
    let limit = paging.limit.unwrap_or(10);
    let cursor = match paging.cursor {
        Some(cursor) => format!("&cursor={cursor}"),
        None => String::new(),
    };

    let session_id = match session_id {
        Some(session_id) => format!("&session_id={session_id}"),
        None => String::new(),
    };

    let result = client
        .requestor
        .client
        .get(format!(
            "{URL}/users/search?keyword={keyword}&limit={limit}{cursor}{session_id}"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<UserSearchByKeyword>(response)
        .await
}

pub async fn authenticated_details(client: &mut Client) -> Result<ClientDetails, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/authenticated"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<ClientDetails>(response).await
}

pub async fn authenticated_age_bracket(client: &mut Client) -> Result<u64, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/authenticated/age-bracket"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "ageBracket")]
        age_bracket: u64,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .age_bracket)
}

pub async fn authenticated_country_code(client: &mut Client) -> Result<String, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/authenticated/country-code"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "countryCode")]
        country_code: String,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .country_code)
}

pub async fn authenticated_roles(client: &mut Client) -> Result<Vec<String>, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/authenticated/roles"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        roles: Vec<String>,
    }

    let response = client.validate_response(result).await?;
    Ok(client
        .requestor
        .parse_json::<Response>(response)
        .await?
        .roles)
}

pub async fn authenticated_app_launch_info(
    client: &mut Client,
) -> Result<ClientAppLaunchInfo, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/users/authenticated/app-launch-info"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<ClientAppLaunchInfo>(response)
        .await
}

pub async fn birthdate(client: &mut Client) -> Result<DateTime, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/birthdate"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "birthDay")]
        day: u8,
        #[serde(rename = "birthMonth")]
        month: u8,
        #[serde(rename = "birthYear")]
        year: i32,
    }

    let response = client.validate_response(result).await?;
    let birthdate = client.requestor.parse_json::<Response>(response).await?;

    Ok(DateTime::from_ymd(
        birthdate.year,
        birthdate.month,
        birthdate.day,
    ))
}

// According to documentation there's supposed to be a password,
// yet the roblox website doesn't use it..
pub async fn set_birthdate(
    client: &mut Client,
    birthdate: DateTime,
    //password: &str,
) -> Result<(), Error> {
    #[derive(Debug, Serialize)]
    struct Request {
        #[serde(rename = "birthDay")]
        day: u8,
        #[serde(rename = "birthMonth")]
        month: u8,
        #[serde(rename = "birthYear")]
        year: i32,
        //pub password: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/birthdate"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            day: birthdate.day(),
            month: birthdate.month(),
            year: birthdate.year(),
            //password,
        })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

pub async fn description(client: &mut Client) -> Result<String, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/description"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "description")]
        value: String,
    }

    let response = client.validate_response(result).await?;
    let description = client.requestor.parse_json::<Response>(response).await?;

    Ok(description.value)
}

pub async fn set_description(client: &mut Client, description: &str) -> Result<(), Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "description")]
        value: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/description"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request { value: description })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

pub async fn gender(client: &mut Client) -> Result<Gender, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/gender"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    #[derive(Debug, Deserialize)]
    struct Response {
        #[serde(rename = "gender")]
        value: u8,
    }

    let response = client.validate_response(result).await?;
    let gender: Response = client.requestor.parse_json(response).await?;

    Ok(Gender::from_u8(gender.value))
}

pub async fn set_gender(client: &mut Client, gender: Gender) -> Result<(), Error> {
    #[derive(Debug, Serialize)]
    struct Request {
        #[serde(rename = "gender")]
        value: u8,
    }

    let result = client
        .requestor
        .client
        .post(format!("{URL}/gender"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            value: gender as u8,
        })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

pub async fn validate_display_name(
    client: &mut Client,
    display_name: &str,
    birthdate: DateTime,
) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .get(format!(
            "{URL}/display-names/validate?displayName={display_name}&birthdate={}",
            birthdate
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

// TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
// there's also currently no way to require id from Client, perhaps we should authenticate
// on from_cookie method, and store the ClientDetails in the Client
pub async fn validate_display_name_by_id(
    client: &mut Client,
    id: u64,
    display_name: &str,
) -> Result<(), Error> {
    let result = client
        .requestor
        .client
        .get(format!(
            "{URL}/users/{id}/display-names/validate?displayName={display_name}"
        ))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}

// TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
// there's also currently no way to require id from Client, perhaps we should authenticate
// on from_cookie method, and store the ClientDetails in the Client
pub async fn set_display_name(
    client: &mut Client,
    id: u64,
    display_name: &str,
) -> Result<(), Error> {
    #[derive(Debug, Serialize)]
    struct Request<'a> {
        #[serde(rename = "newDisplayName")]
        display_name: &'a str,
    }

    let result = client
        .requestor
        .client
        .patch(format!(
            "{URL}/users/{id}/display-names?displayName={display_name}"
        ))
        .headers(client.requestor.default_headers.clone())
        .json(&Request { display_name })
        .send()
        .await;

    client.validate_response(result).await?;
    Ok(())
}
