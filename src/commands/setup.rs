use crate::NecessaryDatas;
use serenity;

#[serenity::framework::standard::macros::command]
pub async fn setup(
    ctx: &serenity::client::Context,
    msg: &serenity::model::channel::Message,
) -> serenity::framework::standard::CommandResult {
    let data = ctx.data.read().await;
    let a = data.get::<NecessaryDatas>().unwrap();
    let a_clone = a.clone();
    print!("{:?}", a_clone);
    msg.reply(ctx, "Hello, world!").await?;
    return Ok(());
}

#[serenity::framework::standard::macros::group]
#[commands(setup)]
struct Setup;
