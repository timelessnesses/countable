use crate::NecessaryDatas;
use serenity;

struct Handler;

impl Handler {
    fn column(&self, num: usize) -> String {
        fn column_helper(num: usize, res: &str) -> String {
            if num > 0 {
                let letters = "abcdefghijklmnopqrstuvwxyz";
                let index = (num - 1) % 26;
                let new_res = format!("{}{}", letters.chars().nth(index).unwrap(), res);
                column_helper((num - 1) / 26, &new_res)
            } else {
                return res.to_string();
            }
        }

        return column_helper(num, "");
    }
}

#[serenity::async_trait]
impl serenity::client::EventHandler for Handler {
    async fn ready(&self, _: serenity::client::Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(
        &self,
        ctx: serenity::client::Context,
        msg: serenity::model::channel::Message,
    ) {
        let data = ctx.data.read().await;
        let necessary = data.get::<NecessaryDatas>().unwrap();
        let database = &necessary.database;

        if msg.author.bot {
            return;
        }

        if msg.guild_id.is_none() {
            return;
        }

        let guild_id = msg.guild_id.unwrap().to_string().parse::<i64>().unwrap();

        let channel = database
            .query(
                "SELECT channel_id FROM config WHERE guild_id = $1",
                &[&guild_id],
            )
            .await;
        if channel.is_err() {
            return;
        }

        let countable_channel = msg.guild(&ctx).unwrap().channels(&necessary.http).await;
        if countable_channel.is_err() {
            return;
        }
        let countable_channel_result = msg.guild(&ctx).unwrap().channels(&necessary.http).await;
        if countable_channel_result.is_err() {
            return;
        }
        let countable_channel = countable_channel_result.unwrap();
        let channel_id = channel.unwrap()[0].get::<usize, String>(0)
            .parse::<u64>()
            .unwrap();
        let countable_channel = countable_channel.get(&serenity::model::id::ChannelId::from(channel_id));
        if msg.channel_id != countable_channel.unwrap().id {
            return;
        }
    }
}
