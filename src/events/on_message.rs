// literally a core of countable
// like seriously
// ! do not edit or do funny stuff with this code else entire bot will break so becareful what you do here
// ! make sure you know what u doing
// ! future timelessnesses

use crate::{Error, Things};

use crate::utils::id::generate_id;
use chrono;
use poise;
use poise::serenity_prelude as poise_serenity;

pub fn column(num: i64, res: &str) -> String {
    if num > 0 {
        let c = ((num - 1) % 26) as usize;
        let mut res_new = String::with_capacity(res.len() + 1);
        res_new.push("abcdefghijklmnopqrstuvwxyz".chars().nth(c).unwrap());
        res_new.push_str(res);
        return column((num - 1) / 26, &res_new);
    } else {
        return String::from(res);
    }
}

struct WHY {
    deez: Option<i64>,
}

pub async fn message(
    ctx: &poise_serenity::Context,
    _event: &poise_serenity::FullEvent,
    things: &Things,
    message: &poise_serenity::Message,
) -> Result<(), Error> {
    let h = WHY { deez: None };
    if message.author.bot {
        return Ok(());
    }
    if message.is_private() {
        return Ok(());
    }
    let guild = message.guild_id.unwrap();
    let db = things.database.as_ref();

    let guild_id = guild.get() as i64;
    let channel_id = message.channel_id.get() as i64;
    let author_id = message.author.id.get() as i64;

    let m = db
        .query(
            "SELECT channel_id FROM config WHERE guild_id = $1",
            &[&guild_id],
        )
        .await
        .unwrap();

    if m.len() == 0 {
        return Ok(());
    }

    if channel_id != m[0].get::<usize, i64>(0) {
        return Ok(());
    }

    let mut previous_person = db
        .query(
            "SELECT previous_person FROM counting WHERE guild_id = $1",
            &[&guild_id],
        )
        .await
        .unwrap();

    let previous_count = db
        .query(
            "SELECT count_number FROM counting WHERE guild_id = $1",
            &[&guild_id],
        )
        .await
        .unwrap();
    let previous_count2 = db
        .query(
            "SELECT count_number FROM counting WHERE guild_id = $1",
            &[&guild_id],
        )
        .await
        .unwrap();

    if previous_person.len() == 0 {
        return Ok(());
    }

    let is_same_person = db
        .query(
            "SELECT is_same_person FROM config WHERE guild_id = $1",
            &[&guild_id],
        )
        .await
        .unwrap();
    let mut x = previous_count[0].get::<usize, i64>(0);
    if previous_person[0].get::<usize, Option<i64>>(0).is_none() {
        db.execute(
            "UPDATE counting SET previous_person = $1 WHERE guild_id = $2",
            &[&author_id, &guild_id],
        )
        .await
        .unwrap();
        previous_person = db
            .query(
                "SELECT previous_person FROM counting WHERE guild_id = $1",
                &[&guild_id],
            )
            .await
            .unwrap();
        x = previous_count[0].get::<usize, i64>(0);
    } else if previous_person[0].get::<usize, Option<i64>>(0).unwrap() == author_id
        && is_same_person[0].get::<usize, bool>(0) == false
    {
        message
            .react(ctx, poise_serenity::ReactionType::Unicode("❌".to_string()))
            .await
            .unwrap();
        let id = generate_id(8);
        let now = chrono::Local::now();
        let embed = poise_serenity::builder::CreateEmbed::new()
            .title("You cannot say alphabet twice in a row! Give this chance to other!")
            .description("You said last and next alphabet. Now the server alphabet is now reset to A.")
            .color(0xff0000)
            .field("Reason", "You can't say last and next alphabet in a row", false)
            .field("Ruined By", message.author.tag(), false)
            .field("Time", now.to_string(), false)
            .field("Fix", "This server did not enabled is_same_person feature, please try not to say it yourself after you said it and wait for other to say it!", false)
            .field("Jump URL", message.link(), false);
        message
            .channel_id
            .send_message(
                ctx,
                poise_serenity::builder::CreateMessage::new().embed(embed),
            )
            .await
            .unwrap();
        let ruined_counts = db
            .query(
                "SELECT ruined_counts FROM user_stats WHERE user_id = $1",
                &[&author_id],
            )
            .await
            .unwrap();
        db.execute(
            "UPDATE user_stats SET ruined_counts=$1 WHERE user_id=$2",
            &[&(ruined_counts[0].get::<usize, i64>(0) + 1), &author_id],
        )
        .await
        .unwrap();
        db.execute(
            "UPDATE counting SET previous_person = $1, count_number =  $2 WHERE guild_id = $3",
            &[&h.deez, &(0 as i64), &guild_id],
        )
        .await
        .unwrap();
        db.execute("INSERT INTO logger (id, guild_id, channel_id, ruiner_id, ruined_jump_url, when_ruined, reason) VALUES ($1, $2, $3, $4, $5, $6, $7)", &[&(id.clone()), &guild_id, &channel_id, &author_id, &message.link(), &now, &"You can't say last and next alphabet in a row"]).await.unwrap();
        let current_count = db
            .query(
                "SELECT count_number, longest_chain FROM counting WHERE guild_id = $1",
                &[&guild_id],
            )
            .await
            .unwrap();
        if current_count.len() == 0 {
            return Ok(());
        }
        check_longest_chain(ctx, message, previous_count, current_count, guild_id, db)
            .await
            .unwrap();
    } else {
        x = previous_count[0].get::<usize, i64>(0);
    }

    let expectation = column(x + 1, "");
    if message.content.to_ascii_lowercase() != expectation {
        message
            .react(ctx, poise_serenity::ReactionType::Unicode("❌".to_string()))
            .await
            .unwrap();
        let id = generate_id(8);
        let now = chrono::Local::now();
        let embed = poise_serenity::builder::CreateEmbed::new()
            .title("You ruined counting due to wrong alphabet order!")
            .description(format!(
                "You said {} but the server expected {}.",
                message.content.to_ascii_lowercase(),
                expectation
            ))
            .color(0xff0000)
            .field("Reason", "You said wrong alphabet order", false)
            .field("Ruined By", message.author.tag(), false)
            .field("Time", now.to_string(), false)
            .field(
                "Fix",
                format!(
                    "You can't say wrong alphabet order, please try to say the next alphabet ({})!",
                    expectation
                ),
                false,
            )
            .field("Jump URL", message.link(), false);
        message
            .channel_id
            .send_message(ctx, poise_serenity::CreateMessage::new().embed(embed))
            .await
            .unwrap();
        let ruined_counts = db
            .query(
                "SELECT ruined_counts FROM user_stats WHERE user_id = $1",
                &[&author_id],
            )
            .await
            .unwrap();
        db.execute(
            "UPDATE user_stats SET ruined_counts=$1 WHERE user_id=$2",
            &[&(ruined_counts[0].get::<usize, i64>(0) + 1), &author_id],
        )
        .await
        .unwrap();
        db.execute("INSERT INTO logger (id, guild_id, channel_id, ruiner_id, ruined_jump_url, when_ruined, reason) VALUES ($1, $2, $3, $4, $5, $6, $7)", &[&(id.clone()), &guild_id, &channel_id, &author_id, &message.link(), &now, &format!("You said wrong alphabet order, please try to say the next alphabet ({})!", expectation)]).await.unwrap();
        db.execute(
            "UPDATE counting SET previous_person = $1, count_number =  $2 WHERE guild_id = $3",
            &[&h.deez, &(0 as i64), &guild_id],
        )
        .await
        .unwrap();
        let current_count = db
            .query(
                "SELECT count_number, longest_chain FROM counting WHERE guild_id = $1",
                &[&guild_id],
            )
            .await
            .unwrap();
        if current_count.len() == 0 {
            return Ok(());
        }
        check_longest_chain(ctx, message, previous_count2, current_count, guild_id, db)
            .await
            .unwrap();
    } else {
        message
            .react(ctx, poise_serenity::ReactionType::Unicode("✅".to_string()))
            .await
            .unwrap();
        let previous_number = db
            .query(
                "SELECT count_number FROM counting WHERE guild_id = $1",
                &[&guild_id],
            )
            .await
            .unwrap();
        db.execute(
            "UPDATE counting SET previous_person = $1, count_number = $2 WHERE guild_id = $3",
            &[
                &author_id,
                &(previous_number[0].get::<usize, i64>(0) + 1),
                &guild_id,
            ],
        )
        .await
        .unwrap();
        let user_stats = db
            .query("SELECT * FROM user_stats WHERE user_id = $1", &[&author_id])
            .await
            .unwrap();
        if user_stats.len() == 0 {
            db.execute("INSERT INTO user_stats (user_id, alphabet_counts, ruined_counts) VALUES ($1, $2, $3)", &[&author_id, &(1 as i64), &(0 as i64)]).await.unwrap();
        } else {
            db.execute(
                "UPDATE user_stats SET alphabet_counts = $1 WHERE user_id = $2",
                &[&(user_stats[0].get::<usize, i64>(1) + 1), &author_id],
            )
            .await
            .unwrap();
        }
    }

    return Ok(());
}

async fn check_longest_chain(
    ctx: &poise_serenity::Context,
    message: &poise_serenity::Message,
    previous_count: Vec<tokio_postgres::Row>,
    current_count: Vec<tokio_postgres::Row>,
    guild_id: i64,
    db: &tokio_postgres::Client,
) -> Result<(), Error> {
    if (previous_count[0].get::<usize, i64>(0) + 1 > current_count[0].get::<usize, i64>(1))
        && previous_count[0].get::<usize, i64>(0) != 0
    {
        let embed = poise_serenity::CreateEmbed::new()
            .title("This server just broke it's personal longest streak!")
            .description(format!(
                "The longest streak was {} ({}) and now it is {} ({})!",
                current_count[0].get::<usize, i64>(1),
                column(current_count[0].get::<usize, i64>(1), ""),
                current_count[0].get::<usize, i64>(0),
                column(current_count[0].get::<usize, i64>(0), "")
            ))
            .color(poise_serenity::Color::DARK_GREEN);
        message
            .channel_id
            .send_message(ctx, poise_serenity::CreateMessage::new().embed(embed))
            .await
            .unwrap();
        db.execute(
            "UPDATE counting SET longest_chain = $1 WHERE guild_id = $2",
            &[&current_count[0].get::<usize, i64>(0), &guild_id],
        )
        .await
        .unwrap();
    }

    // check for global longest streak
    let mut all_guilds = db
        .query("SELECT guild_id, longest_chain FROM counting", &[])
        .await
        .unwrap();
    all_guilds.sort_by(|a, b| b.get::<usize, i64>(1).cmp(&a.get::<usize, i64>(1)));

    let first_rank = all_guilds[0].get::<usize, i64>(1);

    if first_rank < previous_count[0].get::<usize, i64>(0) {
        let guild = message.guild_id.unwrap();
        let guild_name = guild.name(ctx).unwrap();
        let guild_icon = guild.get_preview(ctx).await.unwrap().icon.unwrap();
        let embed = poise_serenity::CreateEmbed::new()
            .title("This server just broke the global longest streak!")
            .description(format!(
                "The longest streak was {} ({}) and now it is {} ({})!",
                current_count[0].get::<usize, i64>(1),
                column(current_count[0].get::<usize, i64>(1), ""),
                current_count[0].get::<usize, i64>(0),
                column(current_count[0].get::<usize, i64>(0), "")
            ))
            .color(poise_serenity::Color::DARK_GREEN)
            .thumbnail(guild_icon.to_string())
            .field("Server", guild_name, false);
        message
            .channel_id
            .send_message(ctx, poise_serenity::CreateMessage::new().embed(embed))
            .await
            .unwrap();
    }

    return Ok(());
}
