extern crate chrono;
extern crate postgres;
extern crate serenity;
extern crate env_logger;
#[macro_use] extern crate log;

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
    // Configure the client with the Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");

    // Create a new instance of the Client.
    let mut client = Client::new(&token, Handler).expect("Error creating the Discord client");

    // Run a background thread to watch for !remindme triggers
    thread::spawn(move || {
        watch_for_reminders();
    });

    // Start a single shard and start listening to events.
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
