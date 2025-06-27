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
    code: u8,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorsJson {
    errors: Vec<ErrorJson>,
}

#[derive(Debug, Deserialize)]
pub struct DataErrorJson {
    #[serde(rename = "isValid")]
    is_valid: bool,
    data: Option<String>, // maybe, i always got null,
    #[serde(rename = "error")]
    message: String,
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

    /// TODO: test if account is terminated
    /// TODO: add reactivate account function
    //pub async fn test_account_status() {}

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
                    self.set_token(&String::from_utf8_lossy(token.as_bytes()).to_string());
                }

                // TODO: some apis like the data api can return an error even with status_code 200
                if code == 200 {
                    return Ok(response);
                }

                // TODO: move this block into the challenge required case
                let challenge = {
                    let challenge_id = response.headers().get(CHALLENGE_ID_HEADER);
                    let challenge_type = response.headers().get(CHALLENGE_TYPE_HEADER);
                    let challenge_metadata_b64 = response.headers().get(CHALLENGE_METADATA_HEADER);

                    if let (Some(id), Some(kind), Some(metadata_b64)) =
                        (challenge_id, challenge_type, challenge_metadata_b64)
                    {
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

                                todo!("Unsupported chef challenge");
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
                };

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
                            code: 0,
                            message: error.message,
                        }],
                    }
                } else {
                    ErrorsJson {
                        errors: vec![ErrorJson {
                            code: 0,
                            message: String::from_utf8_lossy(&bytes).to_string(),
                        }],
                    }
                };

                for error in &errors.errors {
                    dbg!(error);
                }

                match code {
                    400 => {
                        let errors: Vec<ApiError> = errors
                            .errors
                            .iter()
                            .map(|x| match x.message.as_str() {
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

                                _ => ApiError::Unknown(code),
                            })
                            .collect();

                        if errors.len() == 1 {
                            Err(Error::ApiError(errors.first().unwrap().clone()))
                        } else {
                            Err(Error::ApiError(ApiError::Multiple(errors)))
                        }
                    }

                    401 => Err(Error::ApiError(ApiError::Unauthorized)),
                    403 => {
                        let errors: Vec<ApiError> = errors
                            .errors
                            .iter()
                            .map(|x| match x.message.as_str() {
                                "Token Validation Failed"
                                | "XSRF token invalid"
                                | "XSRF Token Validation Failed"
                                | "\"XSRF Token Validation Failed\"" => ApiError::TokenValidation,

                                "PIN is locked." => ApiError::PinIsLocked,
                                "Invalid birthdate change." => ApiError::InvalidBirthdate,

                                "Challenge is required to authorize the request" => {
                                    ApiError::ChallengeRequired(challenge.clone().unwrap())
                                }

                                "Challenge failed to authorize request" => {
                                    ApiError::ChallengeFailed
                                }

                                "You do not have permission to view the owners of this asset." => {
                                    ApiError::PermissionError
                                }

                                "an internal error occurred" => ApiError::Internal,

                                // TODO: add missing challenge duplicate code
                                _ => ApiError::Unknown(code),
                            })
                            .collect();

                        if errors.len() == 1 {
                            Err(Error::ApiError(errors.first().unwrap().clone()))
                        } else {
                            Err(Error::ApiError(ApiError::Multiple(errors)))
                        }
                    }

                    429 => Err(Error::ApiError(ApiError::Ratelimited)),
                    500 => Err(Error::ApiError(ApiError::Internal)),

                    _ => Err(Error::ApiError(ApiError::Unknown(code))),
                }
            }

            Err(error) => Err(Error::ReqwestError(error)),
        }
    }
}
