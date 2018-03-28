extern crate serenity;

mod command_error;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;

use command_error::CommandError;

struct Handler;

fn role(msg: &Message) -> Result<String, CommandError> {
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
        Ok("Role removed!".to_owned())
    } else {
        member.add_role(role)?;
        Ok("Role added!".to_owned())
    }
}

fn roles(msg: &Message) -> Result<String, CommandError> {
    let guild = msg.guild().ok_or("Could not get guild.".to_owned())?;

    let mut roles: Vec<String> = guild
        .read()
        .roles
        .iter()
        .map(|(_, role)| role.name.clone())
        .filter(|role| role != "@everyone")
        .collect();

    roles.sort();

    if roles.len() > 1 {
        let mut v1 = Vec::with_capacity(roles.len() / 2);
        let mut v2 = Vec::with_capacity(roles.len() / 2);
        for (i, s) in roles.into_iter().enumerate() {
            if i % 2 == 0 {
                v1.push(s);
            } else {
                v2.push(format!("{}\n", s));
            }            
        }
        let colwidth = v1.iter().map(|s| s.len()).max().expect("Failed to find max");
        v1 = v1.into_iter().map(|s| format!("{:<width$}  |  ", s, width = colwidth)).collect();

        Ok(format!("```\n{}\n```", v1
            .into_iter()
            .zip(v2)
            .map(|(a, b)| format!("{}{}", a, b))
            .fold(String::new(), |s, n| format!("{}{}", s, n))))
    }
    else {
        Ok(format!("```\n{}\n```", roles
            .into_iter()
            .fold(String::new(), |s, n| format!("{}\n{}", s, n))))
    }
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!role ") {
            let message = match role(&msg) {
                Ok(r) => r,
                Err(e) => format!("{}", e),
            };

            if let Err(why) = msg.channel_id.say(message) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!roles" {
            let message = match roles(&msg) {
                Ok(r) => r,
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
