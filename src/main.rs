use dotenv;
use serenity;
use std;
use tokio;
use tokio_postgres;

mod commands;
struct Handler;

#[serenity::async_trait]
impl serenity::client::EventHandler for Handler {
    async fn ready(&self, _: serenity::client::Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct Necessary {
    database: tokio_postgres::Client,
}

struct NecessaryDatas;

impl serenity::prelude::TypeMapKey for NecessaryDatas {
    type Value = Necessary;
}

impl std::fmt::Debug for Necessary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Necessary {{ database: tokio_postgres::Client }}")
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // database stuff
    let (database, connection) = tokio_postgres::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            std::env::var("COUNTABLE_DB_HOST").expect("No Database Host?"),
            std::env::var("COUNTABLE_DB_PORT").expect("No Database Port?"),
            std::env::var("COUNTABLE_DB_USERNAME").expect("No Database Username?"),
            std::env::var("COUNTABLE_DB_PASSWORD").expect("No password found"), // this is optional
            std::env::var("COUNTABLE_DB_NAME").expect("No Database Name?")
        ),
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // discord stuff

    let token = std::env::var("COUNTABLE_DISCORD_TOKEN").expect("No Discord Token?");
    let http = serenity::http::Http::new(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = std::collections::HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let bot = serenity::framework::standard::StandardFramework::new().configure(|c| {
        c.prefix("a!");
        c.allow_dm(false);
        c.case_insensitivity(true);
        c.ignore_bots(true);
        c.owners(owners);
        return c;
    });

    let mut client = serenity::Client::builder(&token, serenity::prelude::GatewayIntents::all())
        .framework(bot)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    client
        .data
        .write()
        .await
        .insert::<NecessaryDatas>(Necessary { database: database });

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    return (); // why tf you expect ()
}
