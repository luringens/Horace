use serenity::model::channel::Message;
use serenity::model::guild::{Guild, Role};

use command_error::*;

/// Retrieves a list of public roles from a guild.
/// A "public role" is defined here as an role that appears
/// below a role named `vvv public vvv`.
fn get_public_roles(guild: &Guild) -> Vec<&Role> {
    // Get all roles, ignoring `@everyone`
    let mut roles: Vec<&Role> = guild
        .roles
        .iter()
        .map(|(_, role)| role)
        .filter(|role| role.name != "@everyone")
        .collect();

    roles.sort_unstable_by(|r1, r2| r2.position.cmp(&r1.position));

    // Skip until
    roles
        .into_iter()
        .skip_while(|r| r.name != "vvv public vvv")
        .skip(1)
        .collect()
}

/// Takes a `!role ...` command or similar and tries to toggle the authors
/// membership in that role. Only assigns roles from
/// [get_public_roles](get_public_roles).
pub fn role(msg: &Message) -> Result<String, CommandError> {
    let rolename: String = msg.content
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .to_lowercase();

    if rolename.len() == 0 {
        return Err(CommandError::Generic(
            "Please enter a role name.".to_owned(),
        ));
    }

    let guild = msg.guild().ok_or("Could not get guild.".to_owned())?;
    let guildwritelock = guild.read();

    let role = get_public_roles(&guildwritelock);
    let role = role.iter()
        .find(|r| r.name.to_lowercase() == rolename)
        .ok_or("Could not find role.".to_owned())?;

    let mut member = msg.member()
        .ok_or("Could not get guild member from user.".to_owned())?;

    if msg.author.has_role(guildwritelock.id, *role) {
        member.remove_role(*role)?;
        Ok("Role removed!".to_owned())
    } else {
        member.add_role(*role)?;
        Ok("Role added!".to_owned())
    }
}

/// Formats all roles from [get_public_roles](get_public_roles)
/// in a code block, two columns wide.
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
        // Split into two columns, alternating between the two.
        let mut col1 = Vec::with_capacity(roles.len() / 2);
        let mut col2 = Vec::with_capacity(roles.len() / 2);
        for (i, s) in roles.into_iter().enumerate() {
            if i % 2 == 0 {
                col1.push(s);
            } else {
                col2.push(s);
            }
        }

        let colwidth = col1.iter().map(|s| s.len()).max().unwrap_or(5);

        // Print the columns line by line into result.
        let mut result = String::new();
        for i in 0..col1.len() {
            let left = &col1[i];
            let right = if i < col2.len() { &col2[i] } else { "" };
            result.push_str(&format!(
                "{:<width$}  |  {}\n",
                left,
                right,
                width = colwidth
            ));
        }

        // Wrap in a code block to ensure the columns line up.
        Ok(format!("```\n{}\n```", result))
    }
}
