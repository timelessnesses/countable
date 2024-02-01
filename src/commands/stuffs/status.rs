use crate::utils::datetime::format_duration;
use crate::utils::github;
use crate::utils::revision::{get_current_commit, get_last_5_history_commits, BotCurrentStatus};
use crate::utils::versions::{
    get_cargo_version, get_poise_version, get_rust_compiler_version, get_serenity_version,
};
use crate::{Context, Error};
use poise;
use poise::serenity_prelude as poise_serenity;
use sysinfo::{self, Disks};

#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let cpu_percentage = sys.global_cpu_info().cpu_usage() as f32 / sys.cpus().len() as f32;
    let ram_percentage = sys.available_memory() as f32 / sys.total_memory() as f32;
    let d = Disks::new_with_refreshed_list();
    let disk_percentage = d[0].available_space() / d[0].total_space();
    let data = ctx.data();
    let up_when = data.up_when;
    let up_for = chrono::Local::now() - up_when;
    let current_commit = github::convert_hash_id_to_hyperlink(get_current_commit().as_str());
    let mut text = String::new();
    match BotCurrentStatus::get_current_status() {
        BotCurrentStatus::Latest => {
            text = String::from("Latest (commit ")
                + current_commit.to_owned().as_str()
                + &String::from(")"); // HOLY FUCKING DIPSHIT
        }
        BotCurrentStatus::Outdated => {
            text = String::from("Outdated (commit ")
                + current_commit.to_owned().as_str()
                + &String::from(")");
        }
        BotCurrentStatus::Modified => {
            text = String::from("Modified (commit ")
                + current_commit.to_owned().as_str()
                + &String::from(")");
        }
    }
    if text.is_empty() {
        text = "Unknown Error! (Should not be possible)".to_string();
    }
    let embed = poise_serenity::CreateEmbed::new()
        .title("Status")
        .description("Here's some information about the bot.")
        .color(poise_serenity::Color::DARK_GREEN)
        .field("CPU", format!("{}%", cpu_percentage), true)
        .field("RAM", format!("{}%", ram_percentage), true)
        .field("Disk", format!("{}%", disk_percentage), true)
        .field("Uptime", format_duration(up_for), true)
        .field("Rust Compiler Version", get_rust_compiler_version(), true)
        .field("Cargo Version", get_cargo_version(), true)
        .field("Serenity Version", get_serenity_version(), true)
        .field("Poise Version", get_poise_version(), true)
        .field("Bot Revision", text, true)
        .field("Bot Version", env!("CARGO_PKG_VERSION"), true)
        .field("Last 5 Commits", get_last_5_history_commits(), true)
        .timestamp(chrono::prelude::Local::now());
    ctx.send(poise::CreateReply::default().embed(embed))
    .await
    .unwrap();

    return Ok(());
}
