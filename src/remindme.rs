use serenity::model::channel::Message;
use chrono::Duration;

use command_error::*;

static a: &str = "Usage:";

pub fn remindme(msg: &Message) -> Result<String, CommandError> {
    unimplemented!()
}

fn stringToInterval(input: &str) -> Duration {
    unimplemented!()
}