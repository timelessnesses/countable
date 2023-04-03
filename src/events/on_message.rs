use crate::{Context, Error, Things};

use poise;
use poise::serenity_prelude as poise_serenity;

pub async fn message(
    ctx: &poise_serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Things, Error>,
    things: &Things,
    message: &poise_serenity::Message,
) -> Result<(), Error> {
    return Ok(());
}
