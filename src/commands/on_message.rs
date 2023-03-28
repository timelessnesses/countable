use serenity;
use crate::NecessaryDatas;

struct Handler;

#[serenity::async_trait]
impl serenity::client::EventHandler for Handler {
    async fn ready(&self, _: serenity::client::Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: serenity::client::Context, msg: serenity::model::channel::Message) {
        let data = ctx.data.read().await;
        let a = data.get::<NecessaryDatas>().unwrap();
        let database = a.clone();
        print!("{:?}", database);
    }
}