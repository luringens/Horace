use serenity::model::channel::Message;
use serenity::model::guild::{Guild, Role};

use command_error::*;

fn get_public_roles(guild: &Guild) -> Vec<&Role> {
    let mut roles: Vec<&Role> = guild.roles
        .iter()
        .map(|(_, role)| role)
        .filter(|role| role.name != "@everyone")
        .collect();

    roles.sort_unstable_by(|r1, r2| r2.position.cmp(&r1.position));

    let mut result: Vec<&Role> = vec![];
    let mut skipping = true;
    for role in roles {
        if !skipping {
            result.push(role);
        }
        else if role.name == "vvv public vvv" {
            skipping = false;
        }
    }

    result
}

pub fn role(msg: &Message) -> Result<String, CommandError> {
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

pub fn roles(msg: &Message) -> Result<String, CommandError> {
    let guild = msg.guild().ok_or("Could not get guild.".to_owned())?;
    let guild = guild.read();
    
    let mut roles: Vec<&String> = get_public_roles(&guild)
        .iter()
        .map(|role| &role.name)
        .collect();
    
    roles.sort();

    if roles.len() == 0 {
        Ok("No public roles.".to_owned())
    } else {
        let mut col1 = Vec::with_capacity(roles.len() / 2);
        let mut col2 = Vec::with_capacity(roles.len() / 2);
        for (i, s) in roles.into_iter().enumerate() {
            if i % 2 == 0 {
                col1.push(s);
            } else {
                col2.push(format!("{}\n", s));
            }
        }
        let colwidth = col1.iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(5);

        let mut result = String::new();
        for i in 0..col1.len() {
            let left = &col1[i];
            let right = if i < col2.len() { &col2[i] } else { "" };
            result.push_str(&format!("{:<width$}  |  {}", left, right, width = colwidth));
        }

        Ok(format!("```\n{}\n```", result))
    }
}
