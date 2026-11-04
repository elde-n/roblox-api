use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Response, header::HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{
    ApiError, Currency, Error,
    challenge::{
        CHALLENGE_ID_HEADER, CHALLENGE_METADATA_HEADER, CHALLENGE_TYPE_HEADER, Challenge,
        ChallengeMetadata, ChallengeType,
    },
    client::ClientRequestor,
    ratelimit::{
        RATELIMIT_LIMIT_HEADER, RATELIMIT_REMAINING_HEADER, RATELIMIT_RESET_HEADER, Ratelimit,
    },
};

const TOKEN_HEADER: &str = "x-csrf-token";

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorJson {
    //code: u8,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorsJson {
    errors: Vec<ErrorJson>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataErrorJson {
    //#[serde(rename = "isValid")]
    //is_valid: bool,
    //data: Option<String>, // maybe, i always got null,
    #[serde(rename = "error")]
    message: String,
}

fn ratelimit_from_headers(
    limit: Option<&HeaderValue>,
    reset: Option<&HeaderValue>,
    remaining: Option<&HeaderValue>,
) -> Result<Option<Ratelimit>, Error> {
    let (Some(_limit), Some(reset), Some(remaining)) = (limit, reset, remaining) else {
        return Ok(None);
    };

    let remaining = remaining
        .to_str()
        .map_err(|_| Error::InvalidHeader)?
        .parse::<u32>()
        .map_err(|_| Error::InvalidHeader)?;
    let reset_in_seconds = reset
        .to_str()
        .map_err(|_| Error::InvalidHeader)?
        .parse::<u32>()
        .map_err(|_| Error::InvalidHeader)?;

    Ok(Some(Ratelimit {
        remaining,
        reset_in_seconds,
        // TODO: honestly i don't know how to parse this
        windows: Vec::new(),
    }))
}

fn challenge_from_headers(
    id: Option<HeaderValue>,
    kind: Option<HeaderValue>,
    metadata_b64: Option<HeaderValue>,
) -> Result<Option<Challenge>, Error> {
    let (Some(id), Some(kind), Some(metadata_b64)) = (id, kind, metadata_b64) else {
        return Ok(None);
    };

    let kind = ChallengeType::from(kind.to_str().map_err(|_| Error::InvalidHeader)?);

    match kind {
        ChallengeType::Chef => Err(Error::UnsupportedChallenge("chef".to_string())),
        ChallengeType::Captcha => Err(Error::UnsupportedChallenge("captcha".to_string())),
        _ => {
            let metadata = BASE64_STANDARD
                .decode(metadata_b64.to_str().map_err(|_| Error::InvalidHeader)?)
                .map_err(|_| Error::InvalidBase64)?;

            let metadata: ChallengeMetadata =
                serde_json::from_slice(&metadata).map_err(|_| Error::BadJson)?;

            Ok(Some(Challenge {
                id: id.to_str().map_err(|_| Error::InvalidHeader)?.to_string(),
                kind,
                metadata,
            }))
        }
    }
}

impl ClientRequestor {
    fn set_token(&mut self, token: &str) -> Result<(), Error> {
        let token = HeaderValue::from_str(token).map_err(|_| Error::InvalidHeader)?;
        self.default_headers.insert(TOKEN_HEADER, token);
        Ok(())
    }

    pub(crate) async fn validate_response(
        &mut self,
        result: Result<Response, reqwest::Error>,
    ) -> Result<Response, Error> {
        // remove all challenge headers after validation
        self.remove_challenge();

        match result {
            Ok(response) => {
                let code = response.status().as_u16();

                let token = response.headers().get(TOKEN_HEADER);
                if let Some(token) = token {
                    // EVERYTHING must be mutable to do this, perhaps there's another datatype we can use
                    self.set_token(String::from_utf8_lossy(token.as_bytes()).as_ref())?;
                }

                {
                    let limit = response.headers().get(RATELIMIT_LIMIT_HEADER);
                    let reset = response.headers().get(RATELIMIT_RESET_HEADER);
                    let remaining = response.headers().get(RATELIMIT_REMAINING_HEADER);

                    self.ratelimit = ratelimit_from_headers(limit, reset, remaining)?;
                }

                // TODO: some apis like the data api can return an error even with status_code 200
                if code == 200 {
                    return Ok(response);
                }

                let challenge_id = response.headers().get(CHALLENGE_ID_HEADER).cloned();
                let challenge_type = response.headers().get(CHALLENGE_TYPE_HEADER).cloned();
                let challenge_metadata_b64 =
                    response.headers().get(CHALLENGE_METADATA_HEADER).cloned();

                let bytes = response.bytes().await?.to_owned();
                let errors = if let Ok(errors) = serde_json::from_slice::<ErrorsJson>(&bytes) {
                    errors
                } else if let Ok(error) = serde_json::from_slice::<ErrorJson>(&bytes) {
                    ErrorsJson {
                        errors: vec![error],
                    }
                } else if let Ok(error) = serde_json::from_slice::<DataErrorJson>(&bytes) {
                    ErrorsJson {
                        errors: vec![ErrorJson {
                            //code: 0,
                            message: error.message,
                        }],
                    }
                } else {
                    ErrorsJson {
                        errors: vec![ErrorJson {
                            //code: 0,
                            message: String::from_utf8_lossy(&bytes).to_string(),
                        }],
                    }
                };

                match code {
                    401 => Err(Error::ApiError(ApiError::Unauthorized)),
                    429 => Err(Error::ApiError(ApiError::Ratelimited)),
                    500 => Err(Error::ApiError(ApiError::Internal)),
                    _ => {
                        let mut api_errors = Vec::with_capacity(errors.errors.len());
                        for x in &errors.errors {
                            api_errors.push(match x.message.as_str() {
                                // 400 
                                "The asset id is invalid." => ApiError::InvalidAssetId,
                                "Invalid challenge ID." => ApiError::InvalidChallengeId,

                                "User not found." => ApiError::InvalidUser,
                                "The user is invalid or does not exist." => ApiError::InvalidUser,
                                "The target user is invalid or does not exist." => ApiError::InvalidUser,
                                "The user ID is invalid." => ApiError::InvalidUserId,

                                "The gender provided is invalid." => ApiError::InvalidGender,
                                "The two step verification challenge code is invalid." => {
                                    ApiError::InvalidTwoStepVerificationCode
                                }

                                "Invalid display name." => ApiError::InvalidDisplayName,

                                "Request must contain a birthdate" => {
                                    ApiError::RequestMissingArgument("Birthdate".to_string())
                                }

                                "Ascending sort order is not supported for user's favorite games." => {
                                    ApiError::UnsupportedSortOrder
                                }

                                // 403
                                "Token Validation Failed"
                                | "XSRF token invalid"
                                | "XSRF Token Validation Failed"
                                | "\"XSRF Token Validation Failed\"" => ApiError::TokenValidation,

                                "Not authorized." => ApiError::Unauthorized,

                                "Incorrect username or password. Please try again." => {
                                    ApiError::InvalidCredentials
                                }

                                "You must pass the robot test before logging in." => {
                                    ApiError::CaptchaFailed
                                }

                                "Account has been locked. Please request a password reset." => {
                                    ApiError::AccontLocked
                                }

                                "Unable to login. Please use Social Network sign on." => {
                                    ApiError::SocialNetworkLoginRequired
                                }

                                "Account issue. Please contact Support." => {
                                    ApiError::AccountIssue
                                }

                                "Unable to login with provided credentials. Default login is required." => {
                                    ApiError::DefaultLoginRequired
                                }

                                "Received credentials are unverified." => {
                                    ApiError::UnverifiedCredentials
                                }

                                "Existing login session found. Please log out first." => {
                                    ApiError::ExistingLoginSession
                                }

                                "The account is unable to log in. Please log in to the LuoBu app." => {
                                    ApiError::LuoBuAppLoginRequired
                                }

                                "Too many attempts. Please wait a bit." => {
                                    ApiError::Ratelimited
                                }

                                "The account is unable to login. Please log in with the VNG app." => {
                                    ApiError::VNGAppLoginRequired
                                }

                                "PIN is locked." => ApiError::PinIsLocked,

                                "Invalid birthdate change." => ApiError::InvalidBirthdate,

                                "Insufficient Robux funds." => ApiError::NotEnoughFunds(Currency::Robux),

                                // TODO: not sure what this means please use more verbose todo messages
                                // TODO: add missing challenge duplicate code

                                "Challenge is required to authorize the request" => {
                                    let challenge = challenge_from_headers(
                                        challenge_id.clone(),
                                        challenge_type.clone(),
                                        challenge_metadata_b64.clone(),
                                    )?
                                    .ok_or(Error::MissingChallenge)?;
                                    ApiError::ChallengeRequired(challenge)
                                }

                                "Challenge failed to authorize request" => {
                                    ApiError::ChallengeFailed
                                }

                                "You do not have permission to view the owners of this asset." => {
                                    ApiError::PermissionError
                                }

                                "Request Context BrowserTrackerID is missing or invalid." => {
                                    ApiError::InvalidBrowserTrackerId
                                }

                                "Attempt to add non-friends to a conversation" => ApiError::ConversationUserAddFailed,
                                "Attempt to create conversations with non-friends" => ApiError::ConversationCreationFailed,
                                "{\"Error\":\"OneToOne conversations cannot be updated\"}" => ApiError::InvalidConversation,

                                "an internal error occurred" => ApiError::Internal,

                                // 404
                                "Badge is invalid or does not exist." => ApiError::InvalidBadge,


                                // 409
                                "You are already a member of this group." => ApiError::AlreadyInGroup,
                                "You have already requested to join this group." => ApiError::AlreadyInGroupRequests,

                                _ => {
                                    ApiError::Unknown(code, Some(x.message.to_owned()))
                                },
                            });
                        }

                        match api_errors.len() {
                            1 => Err(Error::ApiError(api_errors.swap_remove(0))),
                            _ => Err(Error::ApiError(ApiError::Multiple(api_errors))),
                        }
                    }
                }
            }

            Err(error) => Err(Error::ReqwestError(error)),
        }
    }
}
