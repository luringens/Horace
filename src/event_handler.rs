use std::fmt::Display;
use std::error::Error;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use remindme::*;
use roles::*;
use statistics::*;
use command_error::CommandError;
use connectionpool::ConnectionPool;

pub struct Handler;

static HELP_TEXT: &str = "Usage:
`!ping`: Bot responds with \"Pong!\".
`!help`: Print this message.
`!role`: Join or leave a public role.
`!roles`: Print a list of roles you can join.
`!stats x`: List the 5 most active users for the last `x` days (defaults to 7).
`!remindme x scale message`: Makes the bot send you a PM containing the message after \
x minutes/hours/days/weeks.";

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.lock();
        let cmd = data.get_mut::<ConnectionPool>().unwrap();

        if let Err(e) = save_message_statistic(&msg, &mut cmd) {
            warn!("An error occurred while saving stats: {}", e.description());
        }

        // Early return to avoid lots of strong comparisons.
        if !msg.content.starts_with("!") {
            return;
        }

        match msg.content.split_whitespace().nth(0).unwrap_or("") {
            "!ping" => {
                reply_or_print(&msg, "Pong!");
            }
            "!help" => {
                reply_or_print(&msg, HELP_TEXT);
            }
            "!role" | "!rank" => {
                reply_or_print_result(&msg, role(&msg));
            }
            "!roles" | "!publicroles" | "!ranks" => {
                reply_or_print_result(&msg, roles(&msg));
            }
            "!stats" => {
                reply_or_print_result(&msg, get_message_statistics(&msg, &mut cmd));
            }
            "!remindme" => {
                reply_or_print_result(&msg, remindme(&msg, &mut cmd));
            }
            _ => {}
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn reply_or_print_result(msg: &Message, result: Result<String, CommandError>) {
    match result {
        Ok(r) => reply_or_print(&msg, &r),
        Err(e) => error!("{:?}", e),
    }
}

fn reply_or_print<T: Display>(msg: &Message, text: T) {
    if let Err(why) = msg.channel_id.say(format!("{}", text)) {
        error!("Error sending message: {:?}", why);
    }
}
