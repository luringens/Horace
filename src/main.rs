extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate serenity;
extern crate typemap;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
//mod command_error;
//mod roles;
//mod statistics;
//mod remindme;
//mod connectionpool;

use std::{env, thread};
use serenity::prelude::*;
use serenity::model::{Permissions, gateway::Ready, id::ChannelId};
use serenity::framework::standard::{help_commands, Args, DispatchError, HelpBehaviour, StandardFramework};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use diesel::prelude::*;
use models::*;
use diesel::deserialize::FromSql;
use chrono::Date;
//use statistics::*;
//use command_error::CommandError;
//use remindme::watch_for_reminders;
//use connectionpool::ConnectionPool;

/*struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}*/

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::statistics::dsl::*;

    let connection = establish_connection();
    let results = statistics.load::<Statistic>(&connection)
        .expect("Error loading stats");

    println!("Displaying {} stats", results.len());
    for post in results {
        println!("{}", post.messages);
        println!("{}", post.words);
    }
    return;

    // Configure the Discord client with the bot token in the environment.
    /*
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let mut client = Client::new(&token, Handler).expect("Error creating the Discord client");
    {
        let pool = ConnectionPool::new();
        let mut data = client.data.lock();
        data.insert::<ConnectionPool>(pool);
    }

    // Run a background thread to watch for !remindme triggers
    thread::spawn(move || {
        watch_for_reminders(ConnectionPool::new());
    });

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("?").delimiter(" "))
            .before(|ctx, msg, _command_name| {
                let mut data = ctx.data.lock();
                let mut pool = data.get_mut::<ConnectionPool>().unwrap();

                if let Err(why) = save_message_statistic(&msg, &mut pool) {
                    warn!("An error occurred while saving stats: {:?}", why);
                }

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
                    .bucket("delayed")
                    .cmd(rolesc)
            })
            .command("role", |c| {
                c.desc("Joins or leaves a public role.")
                    .known_as("rank")
                    .cmd(role)
            })
            .command("remindme", |c| {
                c.desc("Have the bot remind you of something.").cmd(remind)
            })
            .command("stats", |c| {
                c.desc("Shows some stats about the most active members.")
                    .required_permissions(Permissions::ADMINISTRATOR)
                    .cmd(stats)
            }), /*.command("ping", |c| c
            .check(owner_check)
            .cmd(ping))*/
    );

    // Start a single shard and start listening to events.
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }*/
}
/*
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

command!(rolesc(_ctx, msg, _args) {
    reply_or_log_error(roles::roles(&msg), &msg.channel_id);
});

command!(role(_ctx, msg, args) {
    let role_name = args.full();
    let result = self::roles::toggle_role(role_name, &msg);
    reply_or_log_error(result, &msg.channel_id);
});

command!(stats(ctx, msg, args) {
    const DEFAULT_DAYS: u32 = 7;
    let days = args.single::<u32>().unwrap_or(DEFAULT_DAYS);
    let mut data = ctx.data.lock();
    let pool = data.get_mut::<ConnectionPool>().expect("Could not get connection pool.");
    let result = get_message_statistics(days, &msg, pool);
    reply_or_log_error(result, &msg.channel_id);
});

command!(remind(ctx, msg, args) {
    let num = args.single::<u32>().unwrap();
    let scale = args.single::<String>().unwrap();

    let mut message = String::new();
    args.multiple::<String>()
        .unwrap()
        .into_iter()
        .for_each(|s| {
            message.push_str(&s);
            message.push_str(" ");
        });

    let mut data = ctx.data.lock();
    let pool = data.get_mut::<ConnectionPool>().expect("Could not get connection pool.");
    let result = remindme::remindme(num, &scale, &message, &msg.author.id, pool);
    reply_or_log_error(result, &msg.channel_id);
});

fn reply_or_log_error(result: Result<String, CommandError>, channel_id: &ChannelId) {
    let result = result.and_then(|r| channel_id.say(r).map_err(|e| CommandError::Serenity(e)));

    if let Err(why) = result {
        error!("Couldn't do !roles: {:?}", why);
    };
}
*/