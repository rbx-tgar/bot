use crate::error::Result;
use twilight_model::gateway::payload::incoming::InteractionCreate;

pub async fn interaction_create(interaction_create: InteractionCreate) -> Result<()> {
    Ok(())
}
