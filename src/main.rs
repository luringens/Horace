extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate serenity;
extern crate typemap;

mod command_error;
mod commands;
mod connectionpool;
mod util;

use serenity::framework::standard::{help_commands, DispatchError, HelpBehaviour, StandardFramework};
use serenity::model::{Permissions, gateway::Ready};
use serenity::prelude::*;
use std::{env, thread};

use commands::*;
use connectionpool::ConnectionPool;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the Discord client with the bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let mut client = Client::new(&token, Handler).expect("Error creating the Discord client");
    {
        let pool = ConnectionPool::new();
        let mut data = client.data.lock();
        data.insert::<ConnectionPool>(pool);
    }

    // Run a background thread to watch for !remindme triggers
    thread::spawn(move || {
        remindme::watch_for_reminders(ConnectionPool::new());
    });

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("?").delimiter(" "))
            .before(|ctx, msg, _command_name| {
                let mut pool = util::get_pool(ctx);

                // Don't reply to PM's as the command is only valid for guilds.
                let guild_id = match msg.guild_id() {
                    Some(id) => id,
                    None => return false,
                };

                // Push additional message and word count to db.
                pool.update_statistics(
                    guild_id,
                    msg.author.id,
                    msg.timestamp.naive_utc().date(),
                    1,
                    msg.content.split_whitespace().count() as i32,
                ).expect("Failed to update statistics");

                true
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => info!("Processed command '{}'", command_name),
                Err(why) => error!("Command '{}' return error {:?}", command_name, why),
            })
            .unrecognised_command(|_ctx, _, unknown_command_name| {
                info!("Could not find command named '{}'", unknown_command_name);
            })
            .on_dispatch_error(|_ctx, msg, error| {
                if let DispatchError::RateLimited(seconds) = error {
                    if let Err(why) = msg.channel_id
                        .say(&format!("Try this again in {} seconds.", seconds))
                    {
                        error!("Failed to send rate limiting error: {:?}", why);
                    }
                }
            })
            .simple_bucket("delayed", 60)
            .command("about", |c| c.cmd(about))
            .customised_help(help_commands::with_embeds, |c| {
                c.individual_command_tip(
                    "Hi! If you want more info about a command, just pass the command as argument.",
                ).command_not_found_text("{} is not a known command.")
                    .suggestion_text("How about this command: {}?")
                    .lacking_permissions(HelpBehaviour::Hide)
                    .lacking_role(HelpBehaviour::Nothing)
                    .wrong_channel(HelpBehaviour::Strike)
            })
            .command("help", |c| {
                c.desc("Lists possible commands.")
                    .bucket("delayed")
                    .cmd(help)
            })
            .command("roles", |c| {
                c.desc("Lists all public roles you can join with `!role`.")
                    .batch_known_as(vec!["ranks", "publicroles"])
                    .guild_only(true)
                    .bucket("delayed")
                    .cmd(roles::publicroles)
            })
            .command("role", |c| {
                c.desc("Joins or leaves a public role.")
                    .guild_only(true)
                    .known_as("rank")
                    .cmd(roles::joinrole)
            })
            .command("remindme", |c| {
                c.desc("Have the bot remind you of something.")
                    .guild_only(true)
                    .cmd(remindme::remind)
            })
            .command("stats", |c| {
                c.desc("Shows some stats about the most active members.")
                    .guild_only(true)
                    .required_permissions(Permissions::ADMINISTRATOR)
                    .cmd(statistics::stats)
            }), /*.command("ping", |c| c
            .check(owner_check)
            .cmd(ping))*/
    );

    // Start a single shard and start listening to events.
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}

command!(about(_ctx, msg, _args) {
    const ABOUT: &str = "A small bot made for the RPS community :)\n\
                         Source code at https://github.com/stisol/Horace";
    if let Err(why) = msg.channel_id.say(ABOUT) {
        println!("Error sending message: {:?}", why);
    }
});

command!(help(_ctx, msg, _args) {
    const HELP: &str = "Usage:\
        `!ping`: Bot responds with \"Pong!\".\
        `!help`: Print this message.\
        `!role`: Join or leave a public role.\
        `!roles`: Print a list of roles you can join.\
        `!stats x`: List the 5 most active users for the last `x` days (defaults to 7).\
        `!remindme x scale message`: Makes the bot send you a PM containing the message after \
        x minutes/hours/days/weeks.";
    if let Err(why) = msg.channel_id.say(HELP) {
        println!("Error sending message: {:?}", why);
    }
});
