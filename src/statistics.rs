use serenity::model::channel::Message;

use std::str::FromStr;

use connectionpool::ConnectionPool;
use command_error::CommandError;

/// Adds a message to the statistics database.
pub fn save_message_statistic(
    msg: &Message,
    pool: &mut ConnectionPool,
) -> Result<(), CommandError> {
    // Don't reply to PM's as the command is only valid for guilds.
    let guild_id = match msg.guild_id() {
        Some(id) => id.0,
        None => return Ok(()),
    };

    let guild_id = format!("{}", guild_id);
    let user_id = format!("{}", msg.author.id.0);
    let date = msg.timestamp.naive_utc().date();
    let messages = 1;
    let words = msg.content.split_whitespace().count() as i32;

    let conn = pool.get_conn()?;
    conn.execute(
        "INSERT INTO statistics (guild_id, user_id, date, messages, words)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT(guild_id, user_id, date) DO UPDATE SET
            messages = statistics.messages + $4,
            words = statistics.words + $5",
        &[&guild_id, &user_id, &date, &messages, &words],
    )?;

    Ok(())
}

/// Get the top ten most active users by word count.
/// Default number of days of activity to look at is 7.
pub fn get_message_statistics(
    days: u32,
    msg: &Message,
    pool: &mut ConnectionPool,
) -> Result<String, CommandError> {
    // Don't reply to PM's as the command is only valid for guilds.
    let guild = match msg.guild() {
        Some(guild) => guild,
        None => return Ok("Can't use statistics in PMs.".to_owned()),
    };
    let guild = guild.read();
    let guild_id = format!("{}", guild.id.0);

    let conn = pool.get_conn()?;
    let rows = &conn.query(
        "SELECT user_id, SUM(messages) as messages, SUM(words) as words FROM statistics
WHERE guild_id = $1 AND date > current_date - CAST ($2 AS INTEGER)
GROUP BY user_id
ORDER BY words DESC
fetch first 10 rows only",
        &[&guild_id, &days],
    )?;

    let mut results = vec![];
    for row in rows.into_iter() {
        let user_id: String = row.get(0);
        let user_id = match u64::from_str(&user_id) {
            Ok(id) => id,
            Err(_) => {
                error!("Failed to parse id {} to int.", user_id);
                continue;
            }
        };

        let user = match guild.member(user_id) {
            Ok(user) => user,
            Err(_) => {
                error!("Failed to look up member from id {}.", user_id);
                continue;
            }
        };

        let messages: i64 = row.get(1);
        let chars: i64 = row.get(2);
        results.push(Stat {
            name: user.display_name().into_owned(),
            messages: messages,
            words: chars,
        });
    }
    results.sort_by(|a, b| b.words.cmp(&a.words));

    let result = format_stats(results);

    Ok(format!(
        "Statistics for the top 10 most active users the last {} days:\
         \n```\n{}\n```",
        days, result
    ))
}

fn format_stats(stat: Vec<Stat>) -> String {
    let name_width = stat.iter().map(|ref s| s.name.len()).max().unwrap_or(5);
    let words_width = stat.iter().map(|ref s| digits(s.words)).max().unwrap_or(5);
    let messages_width = stat.iter()
        .map(|ref s| digits(s.messages))
        .max()
        .unwrap_or(5);

    stat.into_iter()
        .map(|s| {
            format!(
                "{:<nw$} | {:>mw$} messages | {:>ww$} words.",
                s.name,
                s.messages,
                s.words,
                nw = name_width,
                mw = messages_width,
                ww = words_width
            )
        })
        .fold(String::new(), |a, b| format!("{}\n{}", a, b))
}

fn digits(mut number: i64) -> usize {
    let mut digits = 0;
    while number != 0 {
        number /= 10;
        digits += 1;
    }
    return digits;
}

struct Stat {
    pub name: String,
    pub messages: i64,
    pub words: i64,
}
