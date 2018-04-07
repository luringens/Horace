use std::str::FromStr;

use serenity::model::channel::Message;
use chrono::Duration;

use command_error::*;

static a: &str = "Usage:";

/// Ideas for self:
/// Store a list of remind dates in a table in the DB
/// Have a seperate thread check the table once every hour or so,
/// and perform and delete any expired reminders.

pub fn remindme(msg: &Message) -> Result<String, CommandError> {
    let parts: Vec<&str> = msg.content.split_whitespace()
        .skip(1)
        .collect();
    
    if parts.len() != 2 {
        return Err(CommandError::Generic(a.to_owned()))
    }

    let num = match i32::from_str(parts[0]) {
        Ok(i) => i,
        Err(_) => return Err(CommandError::Generic(a.to_owned())),
    };

    match &*(parts[1].to_lowercase()) {
        "days" => {
            
        }
        _ => return Err(CommandError::Generic(a.to_owned())),
    }

    unimplemented!()
}

fn stringToInterval(input: &str) -> Duration {
    unimplemented!()
}