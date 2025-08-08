use crate::{Error, client::Client};

pub const URL: &str = "https://apis.roblox.com/hba-service/v1";

pub async fn server_nonce(client: &mut Client) -> Result<String, Error> {
    let result = client
        .requestor
        .client
        .get(format!("{URL}/getservernonce"))
        .headers(client.requestor.default_headers.clone())
        .send()
        .await;

    let response = client.validate_response(result).await?;

    let bytes = response.bytes().await;
    match bytes {
        Ok(bytes) => {
            let mut nonce = String::from_utf8_lossy(&bytes).to_string();
            nonce.remove(0);
            nonce.remove(nonce.len() - 1);

            Ok(nonce)
        }
        Err(error) => Err(Error::ReqwestError(error)),
    }
}
