use std::fmt::Display;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use roles::*;
use command_error::CommandError;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        match msg.content.split_whitespace().nth(0).unwrap_or("") {
            "!ping" => {
                reply_or_print(&msg, "Pong!");
            }
            "!role" | "!rank" => {
                reply_or_print_result(&msg, role(&msg));
            }
            "!roles" | "!ranks" => {
                reply_or_print_result(&msg, roles(&msg));
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
