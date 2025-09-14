#[cfg(not(feature = "web-socket"))]
fn main() {
    panic!("Feature flag `web-socket` not enabled");
}

#[cfg(feature = "web-socket")]
use std::time::Duration;

#[cfg(feature = "web-socket")]
use dotenvy_macro::dotenv;
#[cfg(feature = "web-socket")]
use roblox_api::{
    api::web_socket::{self, user_hub::MessageRequest},
    client::Client,
};

#[cfg(feature = "web-socket")]
#[tokio::main]
async fn main() {
    let mut client = Client::from_cookie(dotenv!("ROBLOX_COOKIE").into());
    let mut socket = web_socket::user_hub::connect(&mut client).await.unwrap();

    loop {
        if let Ok(result) = socket.read().await {
            for response in &result {
                println!("{:?}", response);
            }
        }

        // i don't know if this is necessary, but it seems like sending this every 10s is good
        socket.send(MessageRequest::Ping).await.unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }

    // socket.close().await.unwrap();
}
