use crate::{Context, Error};
use chrono;
use poise;
use poise::serenity_prelude as poise_serenity;

#[poise::command(prefix_command, slash_command)]
pub async fn setup(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
