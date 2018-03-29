use serenity::model::channel::Message;

use command_error::*;

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
        let colwidth = v1.iter()
            .map(|s| s.len())
            .max()
            .expect("Failed to find max");
        v1 = v1.into_iter()
            .map(|s| format!("{:<width$}  |  ", s, width = colwidth))
            .collect();

        Ok(format!(
            "```\n{}\n```",
            v1.into_iter()
                .zip(v2)
                .map(|(a, b)| format!("{}{}", a, b))
                .fold(String::new(), |s, n| format!("{}{}", s, n))
        ))
    } else {
        Ok(format!(
            "```\n{}\n```",
            roles
                .into_iter()
                .fold(String::new(), |s, n| format!("{}\n{}", s, n))
        ))
    }
}
