use dotenvy_macro::dotenv;
use roblox_api::client::Client;

pub fn client() -> Client {
    Client::from_cookie(dotenv!("ROBLOX_COOKIE").into())
}

#[macro_export]
macro_rules! test_endpoint {
    // No args, no assertion
    ($name:ident, [$($module:ident)::+], $func:ident) => {
        #[tokio::test]
        async fn $name() {
            let mut client = $crate::api::client();
            roblox_api::api::$($module)::+::$func(&mut client)
                .await
                .unwrap();
        }
    };
    // No args, with assertion
    ($name:ident, [$($module:ident)::+], $func:ident => |$resp:pat_param| $body:block) => {
        #[tokio::test]
        async fn $name() {
            let mut client = $crate::api::client();
            let $resp = roblox_api::api::$($module)::+::$func(&mut client)
                .await
                .unwrap();
            $body
        }
    };
    // With args, no assertion
    ($name:ident, [$($module:ident)::+], $func:ident($($arg:expr),* $(,)?)) => {
        #[tokio::test]
        async fn $name() {
            let mut client = $crate::api::client();
            roblox_api::api::$($module)::+::$func(&mut client, $($arg),*)
                .await
                .unwrap();
        }
    };
    // With args, with assertion
    ($name:ident, [$($module:ident)::+], $func:ident($($arg:expr),* $(,)?) => |$resp:pat_param| $body:block) => {
        #[tokio::test]
        async fn $name() {
            let mut client = $crate::api::client();
            let $resp = roblox_api::api::$($module)::+::$func(&mut client, $($arg),*)
                .await
                .unwrap();
            $body
        }
    };
}

#[macro_export]
macro_rules! test_endpoint_noauth {
    // No args, no assertion
    ($name:ident, [$($module:ident)::+], $func:ident) => {
        #[tokio::test]
        async fn $name() {
            let mut client = roblox_api::client::Client::default();
            roblox_api::api::$($module)::+::$func(&mut client)
                .await
                .unwrap();
        }
    };
    // No args, with assertion
    ($name:ident, [$($module:ident)::+], $func:ident => |$resp:pat_param| $body:block) => {
        #[tokio::test]
        async fn $name() {
            let mut client = roblox_api::client::Client::default();
            let $resp = roblox_api::api::$($module)::+::$func(&mut client)
                .await
                .unwrap();
            $body
        }
    };
    // With args, no assertion
    ($name:ident, [$($module:ident)::+], $func:ident($($arg:expr),* $(,)?)) => {
        #[tokio::test]
        async fn $name() {
            let mut client = roblox_api::client::Client::default();
            roblox_api::api::$($module)::+::$func(&mut client, $($arg),*)
                .await
                .unwrap();
        }
    };
    // With args, with assertion
    ($name:ident, [$($module:ident)::+], $func:ident($($arg:expr),* $(,)?) => |$resp:pat_param| $body:block) => {
        #[tokio::test]
        async fn $name() {
            let mut client = roblox_api::client::Client::default();
            let $resp = roblox_api::api::$($module)::+::$func(&mut client, $($arg),*)
                .await
                .unwrap();
            $body
        }
    };
}

mod account_information;
mod asset_delivery;
mod assets;
mod auth;
mod auth_token_service;
mod avatar;
mod badges;
mod develop;
mod economy;
mod friends;
mod gamepasses;
mod games;
mod groups;
mod hba_service;
mod inventory;
mod notifications;
mod platform_chat;
mod premium_features;
mod presence;
mod private_messages;
mod thumbnails;
mod toolbox_service;
mod user_blocking;
mod users;
