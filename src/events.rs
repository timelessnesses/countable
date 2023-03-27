// Serenity event handling stuff that actually exports event trait

use serenity;
use tokio;
use serenity::prelude::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::client::Context, message: serenity::model::channel::Message) {
        if message.content == "!ping" {
            if let Err(why) = message.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    }
}
