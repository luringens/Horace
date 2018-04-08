extern crate chrono;
extern crate postgres;
extern crate serenity;

mod command_error;
mod event_handler;
mod roles;
mod statistics;
mod remindme;

use std::{env, thread};
use serenity::prelude::*;

use remindme::watch_for_reminders;
use event_handler::*;

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Run a background thread to watch for !remindme triggers
    thread::spawn(move || {
        watch_for_reminders();
    });

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
