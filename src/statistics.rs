use postgres::{Connection, TlsMode};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use env;

use std::str::FromStr;

use command_error::CommandError;

pub fn save_message_statistic(msg: &Message) -> Result<(), CommandError> {
    // Don't bother with PMs.
    if msg.guild().is_none() {
        return Ok(())
    }

    let conn_string = env::var("POSTGRES_CONNSTRING").expect("Expected a token in the environment");

    let conn = Connection::connect(conn_string, TlsMode::None)?;

    let guild_id = format!("{}", msg.guild_id().unwrap().0);
    let user_id = format!("{}", msg.author.id.0);
    let messages = 1;
    let words = msg.content.split_whitespace().count() as i32;

    conn.execute(
        "INSERT INTO statistics (guild_id, user_id, messages, words)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT(guild_id, user_id) DO UPDATE SET
            messages = statistics.messages + $3,
            words = statistics.words + $4",
        &[&guild_id, &user_id, &messages, &words],
    )?;

    Ok(())
}

pub fn get_message_statistics(msg: &Message) -> Result<String, CommandError> {
    // Don't bother with PMs.
    if msg.guild().is_none() {
        return Ok("No statistics in PMs.".to_owned())
    }

    let guild = msg.guild().unwrap();
    let guild = guild.read();
    let guild_id = format!("{}", msg.guild_id().unwrap().0);

    let conn_string = env::var("POSTGRES_CONNSTRING").expect("Expected a token in the environment");

    let conn = Connection::connect(conn_string, TlsMode::None)?;

    let mut results = vec![];
    for row in &conn.query(
        "SELECT user_id, messages, words FROM statistics WHERE guild_id = $1",
        &[&guild_id],
    )? {
        let user_id: String = row.get(0);
        
        let user_id = u64::from_str(&user_id).map(|a| UserId::from(a));
        if user_id.is_err() {
            continue;
        }

        let user = guild.member(user_id.unwrap());
        if user.is_err() {
            continue;
        }
        let user = user.unwrap();
        
        let messages: i32 = row.get(1);
        let chars: i32 = row.get(2);
        results.push((user.display_name().into_owned(),
            messages,
            chars
        ));
    }
    results.sort_by(|a, b| b.2.cmp(&a.2));

    let result = results.into_iter()
        .map(|a| format!("{}: {} messages, {} words.", a.0, a.1, a.2))
        .fold(String::new(), |a, b| format!("{}\n{}", a, b));

    Ok(format!("Statistics:\n```\n{}\n```", result))
}
