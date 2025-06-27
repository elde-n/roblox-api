use reqwest::{
    Response,
    header::{self, HeaderMap, HeaderValue},
};

use crate::Error;
use serde::de::DeserializeOwned;

#[derive(Default)]
pub struct Cookie(String);
impl ToString for Cookie {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for Cookie {
    fn from(value: &str) -> Self {
        Self(format!(".ROBLOSECURITY={}", value))
    }
}

#[derive(Default, Debug)]
pub struct ClientRequestor {
    pub(crate) client: reqwest::Client,
    pub(crate) default_headers: HeaderMap,
}

#[derive(Default, Debug)]
pub struct Client {
    pub requestor: ClientRequestor,
}

impl Client {
    pub fn from_cookie(cookie: Cookie) -> Self {
        let client = reqwest::Client::new();
        let mut default_headers = HeaderMap::new();

        default_headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str("Roblox/WinInet").unwrap(),
        );

        default_headers.insert(
            header::COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );

        Client {
            requestor: ClientRequestor {
                client,
                default_headers,
            },
        }
    }
}

impl ClientRequestor {
    pub(crate) async fn parse_json<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, Error> {
        match response.json::<T>().await {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::BadResponse),
        }
    }
}
