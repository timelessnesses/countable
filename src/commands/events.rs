use crate::NecessaryDatas;
use serenity;
use rand;
use chrono;

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

    fn random_id(&self,length: i64) -> String {
        let things = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut id = String::new();
        for _ in 0..length {
            id.push(things.chars().nth(rand::random::<usize>() % things.len()).unwrap());
        }
        return id;
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
        let previous_person = database.query(
            "SELECT previous_person FROM counting WHERE guild_id = $1",
            &[&guild_id],
        ).await.unwrap();

        if previous_person.is_empty() {
            return
        }
        let previous_count = database.query(
            "SELECT count_number FROM counting WHERE guild_id = $1",
            &[&guild_id]
        ).await.unwrap();
        if previous_count.is_empty() {
            return
        }
        let is_same_person = database.query(
            "SELECT is_same_person FROM config WHERE guild_id = $1",
            &[&guild_id],
        ).await.unwrap();
        if is_same_person.is_empty() {
            return
        }

        if previous_person[0].get::<usize,Option<String>>(0) == None {
            database.execute(
                "UPDATE counting SET previous_person = $1 WHERE guild_id = $2",
                &[&msg.author.id.to_string().parse::<i64>().unwrap(), &guild_id],
            ).await.unwrap();
        }
        else if (previous_person[0].get::<usize,Option<String>>(0).unwrap().parse::<i64>().unwrap() == msg.author.id.to_string().parse::<i64>().unwrap()) && (is_same_person[0].get::<usize,Option<bool>>(0).unwrap() == false) {
            msg.react(&necessary.http, serenity::model::channel::ReactionType::Unicode(String::from("❌"))).await.unwrap();
            let id = self.random_id(10);
            let now = chrono::Utc::now();
            let embed = serenity::builder::CreateEmbed::default()
            .title("You can't chain alphabets")
            .color(serenity::utils::Color::RED)
            .timestamp(now)
            .field(
                "Reason",
                "You can't chain multiple alphabets! (Is same person feature is not turned on also note that turning it on makes your progress not shown in the leaderboard!)",
                false
            )
            .field(
                "Log ID",
                id,
                false
            )
            .field(
                "Ruined by",
                msg.author.to_string(),
                false
            )
            .field(
                "Time",
                now,
                false
            )
            .field(
                "How to prevent",
                "Don't saying next alphabets (or same one) by yourself! Give that opportunity to others!",
                false
            );
            msg.send(&necessary.http, |m| {
                m.embed(|e|{
                    e = embed
                })
            }).await;

            return
        }
        else {
            database.execute(
                "UPDATE counting SET previous_person = $1 WHERE guild_id = $2",
                &[&msg.author.id.to_string().parse::<i64>().unwrap(), &guild_id],
            ).await.unwrap();
        }
        println!("{}",previous_person[0].get::<usize,String>(0));

    }
}
