use std::str::FromStr;
use std::{thread, time};

use serenity::model::channel::Message;
use serenity::model::id::UserId;
use chrono::Duration;
use chrono::offset::Utc;
use postgres::{Connection, TlsMode};
use env;

use command_error::*;

static USAGE: &str = "Usage: `!remindme x scale`, where `x` is a number, \
                      and scale is `minutes`, `hours`, `days` or `weeks`.";

/// Stores a reminder in the database.
pub fn remindme(msg: &Message) -> Result<String, CommandError> {
    let user_id = format!("{}", msg.author.id.0);

    // Extract the message from the command.
    let mut message = String::new();
    msg.content.split_whitespace().skip(3).for_each(|s| {
        message.push_str(s);
        message.push_str(" ");
    });
    if message.bytes().len() > 150 {
        return Ok("Message can only be 150 characters long.".to_owned());
    }

    // Parse when the reminder is to be sent.
    // Failure to parse means user error, so return the explanation.
    let interval = match str_to_interval(&msg.content) {
        Ok(i) => i,
        Err(why) => return Ok(why),
    };
    let date = Utc::now()
        .naive_utc()
        .checked_add_signed(interval)
        .ok_or(CommandError::Generic("Date overflow".to_owned()))?;
    
    // And ship it all to the database.
    let conn_string = env::var("POSTGRES_CONNSTRING")?;
    let conn = Connection::connect(conn_string, TlsMode::None)?;
    conn.execute(
        "INSERT INTO reminders (user_id, date, message) VALUES ($1, $2, $3)",
        &[&user_id, &date, &message],
    )?;

    Ok(format!("Reminder set for {} UTC.", date.format("%Y-%m-%d %H:%M:%S")))
}

/// Attempts to parse the interval part from a `!remindme` command.
/// Currently only supports a `x minutes/hours/days/weeks` syntax.
fn str_to_interval(input: &str) -> Result<Duration, String> {
    let parts: Vec<&str> = input.split_whitespace().skip(1).collect();
    if parts.len() < 2 {
        return Err(format!("Not enough arguments.\n{}", USAGE));
    }

    let num = i64::from_str(parts[0])
        .map_err(|_| format!("First argument was not a number.\n{}", USAGE))?;

    match &*(parts[1].to_lowercase()) {
        "minutes" | "minute" => return Ok(Duration::minutes(num)),
        "hours" | "hour" => return Ok(Duration::hours(num)),
        "days" | "day" => return Ok(Duration::days(num)),
        "weeks" | "week" => return Ok(Duration::weeks(num)),
        _ => return Err(format!("Invalid duration scale.\n{}", USAGE)),
    }
}

/// Infinite loop that checks the database periodically for expired reminders.
pub fn watch_for_reminders() -> ! {
    let conn_string = env::var("POSTGRES_CONNSTRING").expect("Expected a token in the environment");
    let conn =
        Connection::connect(conn_string, TlsMode::None).expect("Failed to connect to database");
    loop {
        let rows = &conn.query(
            "SELECT id, user_id, message FROM reminders WHERE date < current_timestamp",
            &[],
        ).expect("Failed to query database.");

        for row in rows.into_iter() {
            let id: i32 = row.get(0);
            let user_id: String = row.get(1);
            let mut message: String = row.get(2);

            // Try to message the user with the reminder.
            // Delete the reminder anyways if unsuccessful to avoid retrying
            // to message deleted accounts forever.
            if let Err(why) = dm_with_message(user_id, message) {
                error!("Error while DM'ing: {}", why);
            }

            conn.execute("DELETE FROM reminders WHERE id = $1", &[&id])
                .expect("Failed to execute database command.");
        }

        thread::sleep(time::Duration::from_secs(60));
    }
}

/// Parses a user_id and sends a reminder to the user.
fn dm_with_message(user_id: String, message: String) -> Result<(), String> {
    let userid = UserId::from_str(&user_id)
        .map_err(|e| format!("Failed to get user id: {}", e))?;

    let user = userid.get()
        .map_err(|e| format!("Failed to get user: {}", e))?;

    let response = if message.is_empty() {
        "Hello! You asked me to remind you of something at this time,\
             but you didn't specify what!".to_owned()
    } else {
        format!(
            "Hello! You asked me to remind you of the following: {}",
            message
        )
    };

    user.direct_message(|m| m.content(&response))
        .map_err(|why| format!("Failed to DM user: {}", why))?;

    Ok(())
}
