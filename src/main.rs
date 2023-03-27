use dotenv::dotenv;
use serenity;
use std::env;
use tokio;
use tokio_postgres;

mod events;

async fn initialize_db(
    host: String,
    port: u8,
    username: String,
    mut password: Option<String>,
    database: String,
) -> Result<tokio_postgres::Client, String> {
    if password == None {
        password = Some("".to_string());
    }
    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={} port={} user={} password={} dbname={}",
            host,
            port,
            username,
            password.unwrap(),
            database
        )
        .as_str(),
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    if let Err(e) = connection.await {
        return Err(e.to_string());
    }

    return Ok(client);
}

async fn run_statement(client: tokio_postgres::Client) -> Result<(), ()> {
    // read the sqls/countable.sql
    let st = tokio::fs::read_to_string("sqls/countable.sql")
        .await
        .unwrap();
    client.batch_execute(st.as_str()).await.unwrap();
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    let host = env::var("COUNTABLE_DB_HOST").expect("COUNTABLE_DB_HOST must be set");
    let port = env::var("COUNTABLE_DB_PORT").expect("COUNTABLE_DB_PORT must be set");
    let username = env::var("COUNTABLE_DB_USERNAME").expect("COUNTABLE_DB_USERNAME must be set");
    let password = env::var("COUNTABLE_DB_PASSWORD").ok();
    let database = env::var("COUNTABLE_DB_DATABASE").expect("COUNTABLE_DB_DATABASE must be set");
    let client = initialize_db(
        host,
        port.parse::<u8>().unwrap(),
        username,
        password,
        database,
    )
    .await
    .unwrap();
    println!("Connected to database");
    run_statement(client).await;

    return Ok(());
}
