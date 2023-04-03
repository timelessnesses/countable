use crate::{Context, Error};
use chrono;
use poise;
use poise::serenity_prelude as poise_serenity;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    let shard = ctx.framework().shard_manager.lock().await;
    let shard = shard.runners.lock().await;
    let ping = shard.get(&poise_serenity::ShardId(ctx.serenity_context().shard_id));
    if ping.is_none() {
        if ping.unwrap().latency.unwrap().as_millis() <= 0 {
            ctx.send(
                |h| {
                    h.embed(|e| {
                        return e.title("Failed to fetch shard!").description("Something is wrong at the bot's end. Please consider send this error to GitHub repository issue!").color(poise_serenity::Color::RED).timestamp(chrono::prelude::Local::now())
                    });
                    return h;
                }
            ).await.unwrap();
        }
    }

    return Ok(());
}
