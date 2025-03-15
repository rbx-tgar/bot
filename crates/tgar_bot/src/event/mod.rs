use crate::error::Result;
use tracing::{error, info};
use twilight_model::gateway::event::Event;

pub mod interaction;

pub fn handle_event(event: Event) {
    let event_kind = event.kind();

    info!("handle_event {event_kind:?}");

    if let Err(error) = match event {
        Event::InteractionCreate(x) => spawn(interaction::interaction_create(*x)),
        _ => Ok(()),
    } {
        error!("error in event handler: {error:?}")
    }
}

fn spawn<F: Future<Output = Result<()>> + Send + 'static>(future: F) -> Result<()> {
    tokio::spawn(async move {
        if let Err(error) = future.await {
            error!("error in async event handler: {error:?}")
        }
    });

    Ok(())
}
