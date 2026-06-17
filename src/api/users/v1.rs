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

endpoint! {
    user_details(id: u64) -> UserDetails {
        GET "{URL}/users/{id}";
    }

    user_username_history(id: u64, paging: Paging<'_>) -> NameHistory {
        GET "{URL}/users/{id}/username-history";
        paging_query { paging, limit = 10 }
        types {
            Username {
                name: String,
            }
            Response {
                data: Vec<Username>,
                next_cursor("nextPageCursor"): Option<String>,
                previous_cursor("previousPageCursor"): Option<String>,
            }
        }
        map |r: Response| {
            let names = r.data.iter().map(|x| x.name.clone()).collect();
            NameHistory {
                names,
                next_cursor: r.next_cursor,
                previous_cursor: r.previous_cursor,
            }
        }
    }

    users_by_id(ids: &[u64], exclude_terminated: bool) -> Vec<UserById> {
        POST "{URL}/users";
        types {
            Request<'a> {
                ids("userIds"): &'a [u64],
                exclude_terminated("excludeBannedUsers"): bool,
            }
            Response {
                data: Vec<UserById>,
            }
        }
        body_serialize { Request { ids, exclude_terminated } }
        map |r: Response| r.data
    }

    users_by_name(names: &[&str], exclude_terminated: bool) -> Vec<UserByName> {
        POST "{URL}/usernames/users";
        types {
            Request<'a> {
                names("usernames"): &'a [&'a str],
                exclude_terminated("excludeBannedUsers"): bool,
            }
            Response {
                data: Vec<UserByName>,
            }
        }
        body_serialize { Request { names, exclude_terminated } }
        map |r: Response| r.data
    }

    // this api seems to be dysfunctional, try using search::omni instead
    search_by_keyword(keyword: &str,session_id: Option<&str>,paging: Paging<'_>) -> UserSearchByKeyword {
        GET "{URL}/users/search";
        paging_query { paging, limit = 10 }
        prelude {
            let session_id_str = session_id.map(|s| s.to_string()).unwrap_or_default();
        }
        query {
            "keyword" => keyword,
            "sessionId" => &session_id_str,
        }
    }

    authenticated_details() -> ClientDetails {
        GET "{URL}/users/authenticated";
    }

    authenticated_age_bracket() -> u64 {
        GET "{URL}/users/authenticated/age-bracket";
        types {
            Response {
                age_bracket("ageBracket"): u64,
            }
        }
        map |r: Response| r.age_bracket
    }

    authenticated_country_code() -> String {
        GET "{URL}/users/authenticated/country-code";
        types {
            Response {
                country_code("countryCode"): String,
            }
        }
        map |r: Response| r.country_code
    }

    authenticated_roles() -> Vec<String> {
        GET "{URL}/users/authenticated/roles";
        types {
            Response {
                roles: Vec<String>,
            }
        }
        map |r: Response| r.roles
    }

    authenticated_app_launch_info() -> ClientAppLaunchInfo {
        GET "{URL}/users/authenticated/app-launch-info";
    }

    birthdate() -> DateTime {
        GET "{URL}/birthdate";
        types {
            Response {
                day("birthDay"): u8,
                month("birthMonth"): u8,
                year("birthYear"): i32,
            }
        }
        map |b: Response| DateTime::from_ymd(b.year, b.month, b.day)
    }

    // According to documentation there's supposed to be a password,
    // yet the roblox website doesn't use it..
    set_birthdate(birthdate: DateTime) -> () {
        POST "{URL}/birthdate";
        types {
            Request {
                day("birthDay"): u8,
                month("birthMonth"): u8,
                year("birthYear"): i32,
            }
        }
        prelude {
            let day = birthdate.day();
            let month = birthdate.month();
            let year = birthdate.year();
        }
        body_serialize { Request { day, month, year } }
    }

    description() -> String {
        GET "{URL}/description";
        types {
            Response {
                value("description"): String,
            }
        }
        map |r: Response| r.value
    }

    set_description(description: &str) -> () {
        POST "{URL}/description";
        types {
            Request<'a> {
                value("description"): &'a str,
            }
        }
        body_serialize { Request { value: description } }
    }

    gender() -> Gender {
        GET "{URL}/gender";
        types {
            Response {
                value("gender"): u8,
            }
        }
        map |g: Response| Gender::from_repr(g.value).expect("failed to parse gender")
    }

    set_gender(gender: Gender) -> () {
        POST "{URL}/gender";
         types {
            SetGenderRequest {
                value("gender"): u8,
            }
        }
        prelude {
            let value = gender as u8;
        }
        body_serialize { SetGenderRequest { value } }

    }

    validate_display_name(display_name: &str, birthdate: DateTime) -> () {
        GET "{URL}/display-names/validate";
        prelude {
            let birthdate = birthdate.to_string();
        }
        query {
            "displayName" => display_name,
            "birthdate" => &birthdate,
        }
    }

    // TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
    // there's also currently no way to require id from Client, perhaps we should authenticate
    // on from_cookie method, and store the ClientDetails in the Client
    validate_display_name_by_id(id: u64, display_name: &str) -> () {
        GET "{URL}/users/{id}/display-names/validate";
        query {
            "displayName" => display_name,
        }
        void
    }

    // TODO: I don't know what `id` is for, as this api only seems to be used for the client only,
    // there's also currently no way to require id from Client, perhaps we should authenticate
    // on from_cookie method, and store the ClientDetails in the Client
    set_display_name(id: u64, display_name: &str) -> () {
        PATCH "{URL}/users/{id}/display-names";
        types {
            Request<'a> {
                display_name("newDisplayName"): &'a str,
            }
        }
        body_serialize { Request { display_name } }
    }
}
