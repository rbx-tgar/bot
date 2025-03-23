use serenity::{Client, gateway::ActivityData, prelude::GatewayIntents};
use tgar_bot_http::create_server;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

struct EventHandler;

impl serenity::prelude::EventHandler for EventHandler {}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("starting tgar bot v{}", env!("CARGO_PKG_VERSION"));

    let mut client = Client::builder(
        env!("DISCORD_BOT_TOKEN").to_owned().parse().unwrap(),
        GatewayIntents::all(),
    )
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
