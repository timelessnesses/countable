mod on_message;
mod on_ready;
use crate::{Error, Things};
pub use on_message::column;
use poise;
use poise::serenity_prelude as poise_serenity;

pub async fn listener(
    ctx: &poise_serenity::Context,
    event: &poise_serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Things, Error>,
    things: &Things,
) -> Result<(), Error> {
    match event {
        poise_serenity::FullEvent::Message { new_message } => {
            on_message::message(ctx, event, things, new_message).await?;
        }
        poise_serenity::FullEvent::Ready { .. } => {
            on_ready::ready(ctx, event, framework, things).await?;
        }
        _ => {}
    }
    return Ok(());
}
