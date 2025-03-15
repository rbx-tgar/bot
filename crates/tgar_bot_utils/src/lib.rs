use once_cell::sync::Lazy;
use rbxcloud::rbx::types::UniverseId;
use rbxcloud::rbx::v1::RbxCloud;
use std::borrow::ToOwned;
use twilight_http::Client;
use twilight_http::client::InteractionClient;
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;

pub static DISCORD_APP_ID: Lazy<Id<ApplicationMarker>> = Lazy::new(|| env!("DISCORD_APP_ID").to_owned().parse().unwrap());
pub static DISCORD_CLIENT: Lazy<Client> = Lazy::new(|| Client::new(env!("DISCORD_BOT_TOKEN").to_owned()));
pub static DISCORD_INTERACTION_CLIENT: Lazy<InteractionClient> = Lazy::new(|| DISCORD_CLIENT.interaction(*DISCORD_APP_ID));

pub static ROBLOX_UNIVERSE_ID: Lazy<UniverseId> = Lazy::new(|| UniverseId(env!("ROBLOX_UNIVERSE_ID").to_owned().parse().unwrap()));
pub static ROBLOX_API_KEY: Lazy<String> = Lazy::new(|| env!("ROBLOX_API_KEY").to_owned());
pub static ROBLOX_CLOUD_CLIENT: Lazy<RbxCloud> = Lazy::new(|| RbxCloud::new(&ROBLOX_API_KEY));
