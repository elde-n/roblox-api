use std::time::SystemTime;

use base64::{Engine, prelude::BASE64_STANDARD};
use p256::{
    ecdsa::{Signature, SigningKey, signature::Signer},
    elliptic_curve::rand_core::OsRng,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{DateTime, Error, api::hba_service, client::Client};

pub const URL: &str = "https://auth.roblox.com/v1";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum LoginType {
    Email,
    #[default]
    Username,
    PhoneNumber,
    EmailOtpSessionToken,
    AuthToken,
    Passkey,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MediaType {
    Email,
    SMS,
    Authenticator,
    RecoveryCode,
    SecurityKey,
    CrossDevice,
    Password,
    Passkey,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RecommendedUsernamesFromDisplayName {
    #[serde(rename = "didGenerateNewUsername")]
    pub new_name_generated: bool,
    #[serde(rename = "suggestedUsernames")]
    pub suggested_names: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub name: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TwoStepVerificationInfo {
    pub media_type: MediaType,
    pub ticket: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: User,
    #[serde(rename = "twoStepVerificationData")]
    pub two_step_verification_info: TwoStepVerificationInfo,
    #[serde(rename = "identityVerificationLoginTicket")]
    pub verification_ticket: String,
    pub is_banned: bool,
    pub should_update_email: bool,
    pub recovery_email: String,
    pub account_blob: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct AuthenticationIntent {
    #[serde(rename = "clientPublicKey")]
    public_key: String,
    #[serde(rename = "clientEpochTimestamp")]
    epoch_timestamp: u64,
    #[serde(rename = "saiSignature")]
    signature: String,
    #[serde(rename = "serverNonce")]
    nonce: String,
}

async fn authentication_intent(client: &mut Client) -> Result<AuthenticationIntent, Error> {
    let nonce = hba_service::v1::server_nonce(client).await?;

    let unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let key = SigningKey::random(&mut OsRng);
    let public_key = BASE64_STANDARD.encode(key.verifying_key().to_sec1_bytes());

    let binding = format!("{}:{}:{}", public_key, unix, nonce);
    let hash = Sha256::digest(binding);

    let signature: Signature = key.sign(&hash[..]);
    let signature = String::from_utf8_lossy(&signature.to_bytes()).to_string();

    Ok(AuthenticationIntent {
        public_key,
        epoch_timestamp: unix,
        signature,
        nonce,
    })
}

pub async fn login(
    client: &mut Client,
    login: &str,
    key: &str,
    login_type: LoginType,
) -> Result<LoginResponse, Error> {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "ctype")]
        login_type: LoginType,
        #[serde(rename = "cvalue")]
        login: &'a str,
        #[serde(rename = "password")]
        key: &'a str,

        #[serde(rename = "secureAuthenticationIntent")]
        authentication_intent: AuthenticationIntent,
    }

    let authentication_intent = authentication_intent(client).await?;
    let result = client
        .requestor
        .client
        .post(format!("{URL}/login"))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            login_type,
            login,
            key,
            authentication_intent,
        })
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client.requestor.parse_json::<LoginResponse>(response).await
}

pub async fn recommended_usernames_from_display_name(
    client: &mut Client,
    display_name: &str,
    birthday: DateTime,
) -> Result<RecommendedUsernamesFromDisplayName, Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Request<'a> {
        display_name: &'a str,
        birthday: &'a str,
    }

    let result = client
        .requestor
        .client
        .post(format!(
            "{URL}/validators/recommendedUsernameFromDisplayName"
        ))
        .headers(client.requestor.default_headers.clone())
        .json(&Request {
            display_name,
            birthday: birthday.to_string().as_str(),
        })
        .send()
        .await;

    let response = client.validate_response(result).await?;
    client
        .requestor
        .parse_json::<RecommendedUsernamesFromDisplayName>(response)
        .await
}
