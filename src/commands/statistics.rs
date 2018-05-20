use serenity::model::channel::Message;

use command_error::CommandError;
use connectionpool::ConnectionPool;
use util;

command!(stats(ctx, msg, args) {
    let days = args.single::<u32>().unwrap_or(7);
    let mut pool = util::get_pool(ctx);
    let result = get_message_statistics(days, &msg, &mut pool)
        .and_then(|r| msg.channel_id.say(r).map_err(|e| CommandError::Serenity(e)));
    if let Err(why) = result {
        error!("Couldn't do !roles: {:?}", why);
    };
});

/// Get the top ten most active users by word count.
/// Default number of days of activity to look at is 7.
pub fn get_message_statistics(
    days: u32,
    msg: &Message,
    pool: &mut ConnectionPool,
) -> Result<String, CommandError> {
    let guild = msg.guild().unwrap();
    let guild = guild.read();

    // Get the stats.
    let statistics = pool.get_statistics(guild.id, days)?;

    // Pretty print.
    let name_width = statistics
        .iter()
        .map(|ref s| s.user_id.display_name().len())
        .max()
        .unwrap_or(5);
    let words_width = statistics
        .iter()
        .map(|ref s| util::digits(s.words))
        .max()
        .unwrap_or(5);
    let messages_width = statistics
        .iter()
        .map(|ref s| util::digits(s.messages))
        .max()
        .unwrap_or(5);

    let result = statistics
        .into_iter()
        .map(|s| {
            format!(
                "{:<nw$} | {:>mw$} messages | {:>ww$} words.",
                s.user_id.display_name(),
                s.messages,
                s.words,
                nw = name_width,
                mw = messages_width,
                ww = words_width
            )
        })
        .fold(String::new(), |a, b| format!("{}\n{}", a, b));

    Ok(format!(
        "Statistics for the top 10 most active users the last {} days:\
         \n```\n{}\n```",
        days, result
    ))
}
