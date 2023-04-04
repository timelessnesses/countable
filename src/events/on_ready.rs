use crate::{Error, Things};

use log;
use poise;
use poise::serenity_prelude as poise_serenity;

pub async fn ready(
    _ctx: &poise_serenity::Context,
    _event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Things, Error>,
    _things: &Things,
) -> Result<(), Error> {
    log::info!("Ready!");
    return Ok(());
}
