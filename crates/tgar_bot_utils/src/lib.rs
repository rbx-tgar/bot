use std::{borrow::ToOwned, string::ToString};

use async_once_cell::OnceCell;
use mongodb::Client;
use once_cell::sync::Lazy;
use tgar_rblx_api::ClientBuilder;

pub mod macros;

static INTERNAL_MONGO_CLIENT: OnceCell<Client> = OnceCell::new();
async fn init_mongo_client() -> Client {
    Client::with_uri_str(env!("MONGODB_URI")).await.expect("error connecting to MongoDB")
}

pub async fn mongo_client() -> &'static Client {
    INTERNAL_MONGO_CLIENT.get_or_init(init_mongo_client()).await
}

pub static ROBLOX_CLIENT: Lazy<tgar_rblx_api::Client> = Lazy::new(|| {
    ClientBuilder::new().roblosecurity(env!("ROBLOSECURITY").to_owned().to_string()).build()
});
