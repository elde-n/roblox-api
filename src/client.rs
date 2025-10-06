use reqwest::{
    Method, Response,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{Error, ratelimit::Ratelimit};

#[derive(Default)]
pub struct Cookie(String);

impl std::fmt::Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub(crate) ratelimit: Option<Ratelimit>,
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

        // For some reason some APIs error if not set
        default_headers.append(
            header::COOKIE,
            HeaderValue::from_str("RBXEventTrackerV2=&browserid=2").unwrap(),
        );

        Client {
            requestor: ClientRequestor {
                client,
                default_headers,
                ratelimit: None,
            },
        }
    }

    pub async fn ensure_token(&mut self) -> Result<(), Error> {
        self.requestor.ensure_token().await
    }

    pub async fn ratelimits(&self) -> Option<Ratelimit> {
        self.requestor.ratelimits().await
    }

    // TODO: test if account is terminated
    // TODO: add reactivate account function
    // pub async fn test_account_status() {}
}

pub(crate) struct ResponseWrapped(Response);
impl ResponseWrapped {
    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T, Error> {
        Ok(self.0.json::<T>().await?)
    }

    pub(crate) async fn bytes(self) -> Result<Vec<u8>, Error> {
        let bytes = self.0.bytes().await;
        match bytes {
            Ok(bytes) => Ok(bytes.to_vec()),
            Err(error) => Err(Error::ReqwestError(error)),
        }
    }
}

impl ClientRequestor {
    pub(crate) async fn parse_json<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, Error> {
        Ok(response.json::<T>().await?)
    }

    pub(crate) async fn request<'a, R: Serialize>(
        &mut self,
        method: Method,
        url: &str,
        request: Option<&'a R>,
        query: Option<&'a [(&'a str, &'a str)]>,
        headers: Option<HeaderMap>,
    ) -> Result<ResponseWrapped, Error> {
        // TODO: use builder outside for this, so we don't need the 3 optionals

        let mut builder = self
            .client
            .request(method, url)
            .headers(headers.unwrap_or(self.default_headers.clone()));

        // Even though sending None works, it might get serialized as null in json, which is a waste of bytes
        if let Some(request) = request {
            builder = builder.json(&request);
        }

        if let Some(query) = query {
            builder = builder.query(&query);
        }

        let response = self.validate_response(builder.send().await).await?;
        Ok(ResponseWrapped(response))
    }
}
