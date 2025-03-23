use poise::{Framework, FrameworkOptions, builtins::register_globally};
use serenity::{Client, gateway::ActivityData, prelude::GatewayIntents};
use tgar_bot_http::create_server;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

use crate::commands::{Data, xp::xp};

mod commands;

struct EventHandler;

impl serenity::prelude::EventHandler for EventHandler {}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("starting tgar bot v{}", env!("CARGO_PKG_VERSION"));

    let poise = Framework::builder()
        .options(FrameworkOptions { commands: vec![xp()], ..Default::default() })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
            })
        })
        .build();

    let mut client = Client::builder(env!("DISCORD_BOT_TOKEN"), GatewayIntents::all())
        .framework(poise)
        .event_handler(EventHandler)
        .activity(ActivityData::custom("Watching over Coruscant"))
        .await
        .expect("failed to create client");

    tokio::spawn(async move {
        if let Err(e) = client.start().await {
            error!("error starting serenity: ${e:?}")
        }
    });

    if let Err(e) = create_server().await {
        error!("error starting server: ${e:?}")
    }
}
