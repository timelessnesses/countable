use crate::utils::datetime::format_duration;
use crate::utils::github;
use crate::utils::revision::{get_current_commit, get_last_x_history_commits, BotCurrentStatus};
use crate::utils::versions::{get_cargo_version, get_rust_compiler_version};
use crate::{Context, Error};
use poise;
use poise::serenity_prelude as poise_serenity;
use sysinfo::{self, CpuExt, DiskExt, SystemExt};

#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await.unwrap();
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let cpu_percentage = sys.global_cpu_info().cpu_usage() as f32 / sys.cpus().len() as f32;
    let ram_percentage = sys.available_memory() as f32 / sys.total_memory() as f32;
    let disk_percentage = sys.disks()[0].available_space() / sys.disks()[0].total_space();
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
    ctx.send(|r| {
        r.embed(|e| {
            return e
                .title("Status")
                .description("Here's some information about the bot.")
                .color(poise_serenity::Color::DARK_GREEN)
                .field("CPU", format!("{}%", cpu_percentage), true)
                .field("RAM", format!("{}%", ram_percentage), true)
                .field("Disk", format!("{}%", disk_percentage), true)
                .field("Uptime", format_duration(up_for), true)
                .field("Rust Compiler Version", get_rust_compiler_version(), true)
                .field("Cargo Version", get_cargo_version(), true)
                .field(
                    "Serenity Version",
                    "Unknown (Still doesn't know how to add this at compile time)",
                    true,
                )
                .field(
                    "Poise Version",
                    "Unknown (Still doesn't know how to add this at compile time)",
                    true,
                )
                .field("Bot Revision", text, true)
                .field("Bot Version", env!("CARGO_PKG_VERSION"), true)
                .field("Last 5 Commits", get_last_x_history_commits(5), true)
                .timestamp(chrono::prelude::Local::now());
        })
    })
    .await
    .unwrap();

    return Ok(());
}
