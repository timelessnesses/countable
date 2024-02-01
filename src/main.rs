// #![allow(stable_features)]
// #![deny(
//     rust_2018_idioms,
//     missing_copy_implementations,
//     noop_method_call,
//     unused
// )]
// #![warn(clippy::pedantic)]
// #![allow(unused_assignments)]

// rewrite of countable using poise

use chrono;
use dotenv;
use fern;
use log;
use poise;
use poise::serenity_prelude as poise_serenity;
use serenity;
use std;
use tokio;
use tokio_postgres; // WHY
mod commands;
mod events;
mod utils;

pub struct Things {
    database: std::sync::Arc<tokio_postgres::Client>,
    up_when: chrono::DateTime<chrono::Local>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Things, Error>;

impl serenity::prelude::TypeMapKey for Things {
    type Value = std::sync::Arc<Things>;
}

fn setup_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y/%m/%d][%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("poise", log::LevelFilter::Debug)
        .level_for("poise_serenity", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("countable.log").unwrap())
        .level_for("tracing", log::LevelFilter::Off)
        .level_for("h2", log::LevelFilter::Off)
        .level_for("rustls", log::LevelFilter::Off)
        .level_for("serenity::client::dispatch", log::LevelFilter::Off)
        .level_for("serenity::http::ratelimiting", log::LevelFilter::Off)
        .level_for("serenity::http:request", log::LevelFilter::Off)
        .apply()
        .unwrap();
}

async fn connect_to_db() -> Result<tokio_postgres::Client, ()> {
    let schema = include_str!("./sqls/countable.sql");
    dotenv::dotenv().ok();
    let (db, conn) = tokio_postgres::connect(
        format!(
            "host={} port={} user={} password={} dbname={}",
            std::env::var("COUNTABLE_POSTGRES_DB_HOST").unwrap_or_else(|_| {
                log::warn!("COUNTABLE_POSTGRES_DB_HOST not set, using localhost");
                return "localhost".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_DB_PORT").unwrap_or_else(|_| {
                log::warn!("COUNTABLE_POSTGRES_DB_PORT not set, using 5432");
                return "5432".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_DB_USER").expect("COUNTABLE_POSTGRES_DB_USER not set"),
            std::env::var("COUNTABLE_POSTGRES_DB_PASSWORD").unwrap_or_else(|_| {
                log::warn!("COUNTABLE_POSTGRES_DB_PASSWORD not set, using empty password");
                return "".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_DB_DATABASE").expect(
                "COUNTABLE_POSTGRES_DB_DATABASE not set neither in enviroment variables nor in .env file")
        )
        .as_str(),
        tokio_postgres::NoTls,
    )
    .await
    .expect("Failed to connect to database. Are you sure its running?");

    tokio::spawn(async move {
        conn.await.expect("Database connection lost");
    });

    db.execute(schema, &[]).await.unwrap();

    return Ok(db);
}

#[tokio::main]
async fn main() {
    setup_logging();
    log::info!("{}", std::env::var("RUSTC_VERSION").unwrap()); // damn
    log::info!("Initialized Logger");
    let db = connect_to_db().await.unwrap();
    log::info!("Connected to database");
    let bot = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                mention_as_prefix: true,
                prefix: Some("c!".into()),
                ..Default::default()
            },
            commands: vec![commands::stuffs()],
            event_handler: |ctx, event, framework, u| {
                std::boxed::Box::pin(events::listener(ctx, event, framework, u))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .unwrap();
                return Ok(Things {
                    database: db.into(),
                    up_when: chrono::Local::now(),
                });
            })
        }) // it works now?????
        .initialize_owners(true)
        .build();
    log::info!("Initialized bot");
    let client = poise_serenity::ClientBuilder::new(
        std::env::var("COUNTABLE_DISCORD_TOKEN").expect(
            "COUNTABLE_DISCORD_TOKEN not set neither in enviroment variables nor in .env file",
        ),
        poise_serenity::GatewayIntents::all(),
    )
    .framework(bot)
    .await
    .unwrap();
    client.start_autosharded().await.unwrap();
    log::info!("Bot stopped");
}
