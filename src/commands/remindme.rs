use std::str::FromStr;
use std::{thread, time};

use chrono::Duration;
use chrono::offset::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serenity::model::id::UserId;
use std::iter;

use connectionpool::ConnectionPool;
use util;

static USAGE: &str = "Usage: `!remindme x scale`, where `x` is a number, \
                      and scale is `minutes`, `hours`, `days` or `weeks`.";

command!(remind(ctx, msg, args) {
    let num = args.single::<i64>().unwrap();
    let scale = args.single::<String>().unwrap();

    let mut message = String::new();
    args.multiple::<String>()
        .unwrap()
        .into_iter()
        .for_each(|s| {
            message.push_str(&s);
            message.push_str(" ");
        });

    let interval = match &*scale.to_lowercase() {
        "minutes" | "minute" => Duration::minutes(num),
        "hours" | "hour" => Duration::hours(num),
        "days" | "day" => Duration::days(num),
        "weeks" | "week" => Duration::weeks(num),
        _ => {
            util::print_or_log_error(&format!("Invalid duration scale.\n{}", USAGE), &msg.channel_id);
            return Ok(());
        }
    };

    let date = match Utc::now().naive_utc().checked_add_signed(interval) {
        Some(v) => v,
        None => {
            util::print_or_log_error("Invalid date (overflow)", &msg.channel_id);
            return Ok(());
        }
    };

    let mut rng = thread_rng();
    let bookmark: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(32)
            .collect();

    util::get_pool(ctx).add_reminder(&msg.author.id, date, &message, &bookmark)?;

    util::print_or_log_error(&format!(
        "Reminder set for {} UTC.\nBookmark: {}",
        date.format("%Y-%m-%d %H:%M"),
        bookmark
    ), &msg.channel_id);
});

/// Infinite loop that checks the database periodically for expired reminders.
pub fn watch_for_reminders(mut pool: ConnectionPool) -> ! {
    loop {
        thread::sleep(time::Duration::from_secs(60));

        // Get expired reminders.
        let reminders = match pool.get_expired_reminders() {
            Ok(rows) => rows,
            Err(why) => {
                error!("Failed to get reminders: {:?}", why);
                continue;
            }
        };

        // Delete the reminder no matter if the reminder was sent sucessfully
        // or not to avoid retrying to message deleted accounts forever.
        for reminder in reminders.iter() {
            if let Err(why) = pool.delete_reminder(reminder.id) {
                warn!("Failed to delete reminder: {}", why);
            };
        }

        // Send all reminders.
        for reminder in reminders.into_iter() {
            if let Err(why) = dm_with_message(reminder.user_id, reminder.message) {
                error!("Error while DM'ing: {}", why);
            }
        }
    }
}

/// Parses a user_id and sends a reminder to the user.
fn dm_with_message(user_id: String, message: String) -> Result<(), String> {
    let userid = UserId::from_str(&user_id).map_err(|e| format!("Failed to get user id: {}", e))?;

    let user = userid
        .get()
        .map_err(|e| format!("Failed to get user: {}", e))?;

    let response = if message.is_empty() {
        "Hello! You asked me to remind you of something at this time,\n\
         but you didn't specify what!"
            .to_owned()
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
