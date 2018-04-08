use std::fmt::Display;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use remindme::*;
use roles::*;
use statistics::*;
use command_error::CommandError;

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
    fn message(&self, _: Context, msg: Message) {
        if let Err(e) = save_message_statistic(&msg) {
            println!("An error occurred while saving stats: {}", e);
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
                reply_or_print_result(&msg, get_message_statistics(&msg));
            }
            "!remindme" => {
                reply_or_print_result(&msg, remindme(&msg));
            }
            _ => {}
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn reply_or_print_result(msg: &Message, result: Result<String, CommandError>) {
    let message = match result {
        Ok(r) => r,
        Err(e) => format!("{}", e),
    };

    reply_or_print(&msg, &message);
}

fn reply_or_print<T: Display>(msg: &Message, text: T) {
    if let Err(why) = msg.channel_id.say(format!("{}", text)) {
        println!("Error sending message: {:?}", why);
    }
}
