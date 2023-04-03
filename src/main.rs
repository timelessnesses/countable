// rewrite of countable using poise

use chrono;
use dotenv;
use poise;
use poise::serenity_prelude as poise_serenity;
use std;
use tokio;
use tokio_postgres;
// commands and events import

mod commands;
mod events;
mod utils;

pub struct Things {
    database: std::sync::Arc<tokio_postgres::Client>,
    up_when: chrono::DateTime<chrono::Local>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Things, Error>;

impl poise_serenity::TypeMapKey for Things {
    type Value = std::sync::Arc<Things>;
}

async fn connect_to_db() -> Result<tokio_postgres::Client, ()> {
    dotenv::dotenv().ok();
    let (db, conn) = tokio_postgres::connect(
        format!(
            "host={} port={} user={} password={} database={}",
            std::env::var("COUNTABLE_POSTGRES_HOST").unwrap_or_else(|_| {
                return "localhost".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_PORT").unwrap_or_else(|_| {
                return "5432".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_USER").unwrap(),
            std::env::var("COUNTABLE_POSTGRES_PASSWORD").unwrap_or_else(|_| {
                return "".to_string();
            }),
            std::env::var("COUNTABLE_POSTGRES_DATABASE").unwrap()
        )
        .as_str(),
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    if let Err(_) = conn.await {
        return Err(());
    }

    return Ok(db);
}

#[tokio::main]
async fn main() {
    let db = connect_to_db().await.unwrap();
    let bot = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                mention_as_prefix: true,
                prefix: Some("c!".into()),
                ..Default::default()
            },
            commands: vec![commands::stuffs()],
            ..Default::default()
        })
        .token(std::env::var("COUNTABLE_DISCORD_TOKEN").expect(
            "COUNTABLE_DISCORD_TOKEN not set neither in enviroment variables nor in .env file",
        ))
        .intents(poise_serenity::GatewayIntents::all())
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
        .initialize_owners(true);
    bot.run_autosharded().await.unwrap();
}
