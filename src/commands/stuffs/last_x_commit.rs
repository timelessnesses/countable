use crate::utils::revision::get_last_x_history_commits;
use crate::{Context, Error};
use poise;
use poise::serenity_prelude as poise_serenity;

#[poise::command(prefix_command, slash_command)]
pub async fn last_x_commit(ctx: Context<'_>, x: u128) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    let last_x_commit = get_last_x_history_commits(x);
    if last_x_commit.len() <= 0 {
        ctx.send(
            |h| {
                h.embed(|e| {
                    return e.title("Failed to fetch last x commit!").description("Something is wrong at the bot's end. Please consider send this error to GitHub repository issue!").color(poise_serenity::Color::RED).timestamp(chrono::prelude::Local::now())
                });
                return h;
            }
        ).await.unwrap();
    }
    ctx.send(|h| {
        h.embed(|e| {
            return e
                .title("Last x commit")
                .description(format!("```{}```", last_x_commit))
                .color(poise_serenity::Color::BLUE)
                .timestamp(chrono::prelude::Local::now());
        });
        return h;
    })
    .await
    .unwrap();
    return Ok(());
}
