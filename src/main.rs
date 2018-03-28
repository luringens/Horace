extern crate serenity;

mod command_error;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;

use command_error::CommandError;

struct Handler;

fn role(msg: &Message) -> Result<(), CommandError> {
    let rolename = msg.content.trim_left_matches("!role ");

    if rolename.len() == 0 {
        return Err(CommandError::Generic(
            "Please enter a role name.".to_owned(),
        ));
    }

    let guild = msg.guild().ok_or("Could not get guild.".to_owned())?;
    let guildwritelock = guild.read();

    let &(_, role) = &(*guildwritelock)
        .roles
        .iter()
        .find(|&(_, role)| role.name == rolename)
        .ok_or("Could not find role.".to_owned())?;

    let mut member = msg.member()
        .ok_or("Could not get guild member from user.".to_owned())?;

    if msg.author.has_role(guildwritelock.id, role) {
        member.remove_role(role)?;
    } else {
        member.add_role(role)?;
    }

    Ok(())
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!role ") {
            let message = match role(&msg) {
                Ok(()) => "Role added!".to_owned(),
                Err(e) => format!("{}", e),
            };

            if let Err(why) = msg.channel_id.say(message) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
