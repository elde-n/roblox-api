<div align="center">
<h1>roblox-api</h1>

Roblox web API bindings written in rust.
</div>

## Breaking changes
In this library breaking changes occur often as it's in alpha.
Roblox also sometimes changes apis although that's rare.

## Example
```rs
use roblox_api::{api::users, client::Client};

#[tokio::main]
async fn main() {
	let mut client = Client::default();

	let user = users::v1::user_details(&mut client, 1).await.unwrap();
	println!("Name: {}", user.name);
}
```

## Contributing
Before writing any pull-requests please write an issue first.
