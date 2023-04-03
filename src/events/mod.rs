mod on_message;
use crate::{Context, Error, Things};
use poise;
use poise::serenity_prelude as poise_serenity;

pub async fn listener(
    ctx: &poise_serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Things, Error>,
    things: &Things,
) -> Result<(), Error> {
    match event {
        poise::Event::Message { new_message } => {
            on_message::message(ctx, event, framework, things, new_message).await?;
        }
        _ => {}
    }
    return Ok(());
}
