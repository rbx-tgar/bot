use once_cell::sync::Lazy;
use tgar_bot_utils::{DISCORD_INTERACTION_CLIENT, ROBLOX_CLOUD_CLIENT};
use tracing::{Level, info, warn};
use tracing_subscriber::FmtSubscriber;
use twilight_gateway::{ConfigBuilder, EventTypeFlags, Shard, StreamExt};
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{Activity, ActivityType, Status};
use twilight_model::gateway::{CloseFrame, Intents, ShardId};
use crate::event::handle_event;

mod error;
mod event;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("starting tgar bot v{}", env!("CARGO_PKG_VERSION"));

    Lazy::force(&DISCORD_INTERACTION_CLIENT);
    Lazy::force(&ROBLOX_CLOUD_CLIENT);

    let config = ConfigBuilder::new(
        env!("DISCORD_BOT_TOKEN").to_string(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    )
    .presence(
        UpdatePresencePayload::new(
            vec![Activity {
                application_id: None,
                assets: None,
                buttons: vec![],
                created_at: None,
                details: None,
                emoji: None,
                flags: None,
                id: None,
                instance: None,
                kind: ActivityType::Custom,
                name: "child eater".into(),
                party: None,
                secrets: None,
                state: Some("Watching over Coruscant".into()),
                timestamps: None,
                url: None,
            }],
            false,
            None,
            Status::Online,
        )
        .unwrap(),
    )
    .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);
    let sender = shard.sender();

    tokio::spawn(async move {
        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let Ok(event) = item else {
                warn!(source = ?item.unwrap_err(), "error receiving event");
                continue;
            };
            
            handle_event(event);
        }
    });

    tokio::signal::ctrl_c().await.unwrap();

    info!("shutdown signal received");

    sender.close(CloseFrame::NORMAL).unwrap();

    info!("closed shard, shutting down")
}
