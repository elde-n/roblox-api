use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Response, header::HeaderValue};
use serde::Deserialize;

use crate::{
    ApiError, Error,
    api::auth,
    challenge::{
        CHALLENGE_ID_HEADER, CHALLENGE_METADATA_HEADER, CHALLENGE_TYPE_HEADER, Challenge,
        ChallengeMetadata, ChallengeType, ChefChallengeMetadata,
    },
    client::Client,
};

const TOKEN_HEADER: &str = "x-csrf-token";

#[derive(Debug, Deserialize)]
pub struct ErrorJson {
    //code: u8,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorsJson {
    errors: Vec<ErrorJson>,
}

#[derive(Debug, Deserialize)]
pub struct DataErrorJson {
    //#[serde(rename = "isValid")]
    //is_valid: bool,
    //data: Option<String>, // maybe, i always got null,
    #[serde(rename = "error")]
    message: String,
}

fn challenge_from_headers(
    id: Option<HeaderValue>,
    kind: Option<HeaderValue>,
    metadata_b64: Option<HeaderValue>,
) -> Option<Challenge> {
    if let (Some(id), Some(kind), Some(metadata_b64)) = (id, kind, metadata_b64) {
        let kind = ChallengeType::from(kind.to_str().unwrap());
        match kind {
            ChallengeType::Chef => {
                let _metadata: ChefChallengeMetadata = serde_json::from_slice(
                    BASE64_STANDARD
                        .decode(metadata_b64.to_str().unwrap())
                        .unwrap()
                        .as_slice(),
                )
                .unwrap();

                todo!("Unsupported challenge-type: \"chef\"");
            }

            ChallengeType::Captcha => {
                todo!("Unsupported challenge-type: \"captcha\"")
            }

            _ => {
                let metadata: ChallengeMetadata = serde_json::from_slice(
                    BASE64_STANDARD
                        .decode(metadata_b64.to_str().unwrap())
                        .unwrap()
                        .as_slice(),
                )
                .unwrap();

                Some(Challenge {
                    id: id.to_str().unwrap().to_string(),
                    kind,
                    metadata,
                })
            }
        }
    } else {
        None
    }
}

impl Client {
    fn set_token(&mut self, token: &str) {
        self.requestor
            .default_headers
            .insert(TOKEN_HEADER, HeaderValue::from_str(token).unwrap());
    }

    // NOTE: this doesn't work on all apis, since some apis expect a custom token,
    // you'll know which ones are affected based on the `TokenValidation` error
    pub async fn ensure_token(&mut self) -> Result<(), Error> {
        let result = self
            .requestor
            .client
            .post(format!("{}//", auth::URL))
            .headers(self.requestor.default_headers.clone())
            .send()
            .await;

        let result = self.validate_response(result).await;

        if let Err(Error::ApiError(ApiError::TokenValidation)) = result {
            return Ok(());
        }

        if result.is_err() {
            return Err(result.err().unwrap());
        }

        Ok(())
    }

    // TODO: test if account is terminated
    // TODO: add reactivate account function
    // pub async fn test_account_status() {}

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
                    self.set_token(String::from_utf8_lossy(token.as_bytes()).as_ref());
                }

                // TODO: some apis like the data api can return an error even with status_code 200
                if code == 200 {
                    return Ok(response);
                }

                let challenge_id = response.headers().get(CHALLENGE_ID_HEADER).cloned();
                let challenge_type = response.headers().get(CHALLENGE_TYPE_HEADER).cloned();
                let challenge_metadata_b64 =
                    response.headers().get(CHALLENGE_METADATA_HEADER).cloned();

                let bytes = response.bytes().await.unwrap().to_owned();
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
                        let errors: Vec<ApiError> = errors
                            .errors
                            .iter()
                            .map(|x| match x.message.as_str() {
                                // 400 
                                "The asset id is invalid." => ApiError::InvalidAssetId,
                                "Invalid challenge ID." => ApiError::InvalidChallengeId,
                                "User not found." => ApiError::UserNotFound,
                                "The user ID is invalid." => ApiError::InvalidUserId,
                                "The gender provided is invalid." => ApiError::InvalidGender,
                                "The two step verification challenge code is invalid." => {
                                    ApiError::InvalidTwoStepVerificationCode
                                }

                                "Invalid display name." => ApiError::InvalidDisplayName,

                                "Request must contain a birthdate" => {
                                    ApiError::RequestMissingArgument("Birthdate".to_string())
                                }

                                // 403
                                "Token Validation Failed"
                                | "XSRF token invalid"
                                | "XSRF Token Validation Failed"
                                | "\"XSRF Token Validation Failed\"" => ApiError::TokenValidation,

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

                                // TODO: not sure what this means please use more verbose todo messages
                                // TODO: add missing challenge duplicate code

                                "Challenge is required to authorize the request" => {
                                    let challenge = challenge_from_headers(
                                        challenge_id.clone(),
                                        challenge_type.clone(),
                                        challenge_metadata_b64.clone(),
                                    );
                                    ApiError::ChallengeRequired(challenge.unwrap())
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

                                "an internal error occurred" => ApiError::Internal,

                                // 409
                                "You are already a member of this group." => ApiError::AlreadyInGroup,
                                "You have already requested to join this group." => ApiError::AlreadyInGroupRequests,

                                _ => {
                                    ApiError::Unknown(code)
                                },
                            }).collect();

                        if errors.len() == 1 {
                            Err(Error::ApiError(errors.first().unwrap().clone()))
                        } else {
                            Err(Error::ApiError(ApiError::Multiple(errors)))
                        }
                    }
                }
            }

            Err(error) => Err(Error::ReqwestError(error)),
        }
    }
}
