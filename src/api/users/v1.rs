use crate::{DateTime, Gender, Paging, endpoint};

pub const URL: &str = "https://users.roblox.com/v1";

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClientDetails {
    pub id: u64,
    pub name: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClientAppLaunchInfo {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub is_premium: bool,
    pub country_code: String,
    pub age_bracket: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub created: DateTime,
    #[serde(rename = "isBanned")]
    pub is_terminated: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserById {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserByName {
    pub id: u64,
    pub name: String,
    #[serde(rename = "requestedUsername")]
    pub requested_name: String,
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserByKeyword {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "previousUsernames")]
    pub previous_names: Vec<String>,
    #[serde(rename = "hasVerifiedBadge")]
    pub is_verified: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NameHistory {
    pub names: Vec<String>,
    pub next_cursor: Option<String>,
    pub previous_cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserSearchByKeyword {
    #[serde(rename = "data")]
    pub users: Vec<UserByKeyword>,
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    pub previous_cursor: Option<String>,
}

#[endpoint(GET, "{URL}/users/{id}")]
async fn user_details(id: u64) -> UserDetails {}

#[endpoint(GET, "{URL}/users/{id}/username-history")]
async fn user_username_history(id: u64, paging: Paging<'_>) -> NameHistory {
    paging!(paging, limit = 10);
    #[derive(Serialize, Deserialize)]
    struct Username {
        pub name: String,
    }
    #[derive(Deserialize)]
    struct Response {
        pub data: Vec<Username>,
        #[serde(rename = "nextPageCursor")]
        pub next_cursor: Option<String>,
        #[serde(rename = "previousPageCursor")]
        pub previous_cursor: Option<String>,
    }
    map! { |r: Response| {
        let names = r.data.iter().map(|x| x.name.clone()).collect();
        NameHistory {
            names,
            next_cursor: r.next_cursor,
            previous_cursor: r.previous_cursor,
        }
    } }
}

#[endpoint(POST, "{URL}/users")]
async fn users_by_id(ids: &[u64], exclude_terminated: bool) -> Vec<UserById> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "userIds")]
        pub ids: &'a [u64],
        #[serde(rename = "excludeBannedUsers")]
        pub exclude_terminated: bool,
    }
    #[derive(Deserialize)]
    struct Response {
        pub data: Vec<UserById>,
    }
    body! { Request { ids, exclude_terminated } }
    map! { |r: Response| r.data }
}

#[endpoint(POST, "{URL}/usernames/users")]
async fn users_by_name(names: &[&str], exclude_terminated: bool) -> Vec<UserByName> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "usernames")]
        pub names: &'a [&'a str],
        #[serde(rename = "excludeBannedUsers")]
        pub exclude_terminated: bool,
    }
    #[derive(Deserialize)]
    struct Response {
        pub data: Vec<UserByName>,
    }
    body! { Request { names, exclude_terminated } }
    map! { |r: Response| r.data }
}

// this api seems to be dysfunctional, try using search::omni instead

#[endpoint(GET, "{URL}/users/search")]
async fn search_by_keyword(
    keyword: &str,
    session_id: Option<&str>,
    paging: Paging<'_>,
) -> UserSearchByKeyword {
    paging!(paging, limit = 10);
    let session_id_str = session_id.map(|s| s.to_string()).unwrap_or_default();

    query! {
        "keyword" => keyword,
        "sessionId" => &session_id_str,
    }
}

#[endpoint(GET, "{URL}/users/authenticated")]
async fn authenticated_details() -> ClientDetails {}

#[endpoint(GET, "{URL}/users/authenticated/age-bracket")]
async fn authenticated_age_bracket() -> u64 {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "ageBracket")]
        pub age_bracket: u64,
    }
    map! { |r: Response| r.age_bracket }
}

#[endpoint(GET, "{URL}/users/authenticated/country-code")]
async fn authenticated_country_code() -> String {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "countryCode")]
        pub country_code: String,
    }
    map! { |r: Response| r.country_code }
}

#[endpoint(GET, "{URL}/users/authenticated/roles")]
async fn authenticated_roles() -> Vec<String> {
    #[derive(Deserialize)]
    struct Response {
        pub roles: Vec<String>,
    }
    map! { |r: Response| r.roles }
}

#[endpoint(GET, "{URL}/users/authenticated/app-launch-info")]
async fn authenticated_app_launch_info() -> ClientAppLaunchInfo {}

#[endpoint(GET, "{URL}/birthdate")]
async fn birthdate() -> DateTime {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "birthDay")]
        pub day: u8,
        #[serde(rename = "birthMonth")]
        pub month: u8,
        #[serde(rename = "birthYear")]
        pub year: i32,
    }
    map! { |b: Response| DateTime::from_ymd(b.year, b.month, b.day) }
}

// According to documentation there's supposed to be a password,
// yet the roblox website doesn't use it..

#[endpoint(POST, "{URL}/birthdate")]
async fn set_birthdate(birthdate: DateTime) -> () {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename = "birthDay")]
        pub day: u8,
        #[serde(rename = "birthMonth")]
        pub month: u8,
        #[serde(rename = "birthYear")]
        pub year: i32,
    }
    let day = birthdate.day();
    let month = birthdate.month();
    let year = birthdate.year();

    body! { Request { day, month, year } }
}

#[endpoint(GET, "{URL}/description")]
async fn description() -> String {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "description")]
        pub value: String,
    }
    map! { |r: Response| r.value }
}

#[endpoint(POST, "{URL}/description")]
async fn set_description(description: &str) -> () {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "description")]
        pub value: &'a str,
    }
    body! { Request { value: description } }
}

#[endpoint(GET, "{URL}/gender")]
async fn gender() -> Gender {
    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "gender")]
        pub value: u8,
    }
    map! { |g: Response| Gender::from_repr(g.value).expect("failed to parse gender") }
}

#[endpoint(POST, "{URL}/gender")]
async fn set_gender(gender: Gender) -> () {
    #[derive(Serialize, Deserialize)]
    struct SetGenderRequest {
        #[serde(rename = "gender")]
        pub value: u8,
    }
    let value = gender as u8;

    body! { SetGenderRequest { value } }
}

#[endpoint(GET, "{URL}/display-names/validate")]
async fn validate_display_name(display_name: &str, birthdate: DateTime) -> () {
    let birthdate = birthdate.to_string();

    query! {
        "displayName" => display_name,
        "birthdate" => &birthdate,
    }
}

// TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
// there's also currently no way to require id from Client, perhaps we should authenticate
// on from_cookie method, and store the ClientDetails in the Client

#[endpoint(GET, "{URL}/users/{id}/display-names/validate")]
async fn validate_display_name_by_id(id: u64, display_name: &str) -> () {
    query! {
        "displayName" => display_name,
    }
    void!();
}

// TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
// there's also currently no way to require id from Client, perhaps we should authenticate
// on from_cookie method, and store the ClientDetails in the Client

#[endpoint(PATCH, "{URL}/users/{id}/display-names")]
async fn set_display_name(id: u64, display_name: &str) -> () {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "newDisplayName")]
        pub display_name: &'a str,
    }
    body! { Request { display_name } }
}
