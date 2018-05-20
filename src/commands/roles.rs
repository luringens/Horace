use serenity::model::guild::{Guild, Role};

use util::print_or_log_error;

command!(publicroles(_ctx, msg, _args) {
    let guild = msg.guild().unwrap();
    let guild = guild.read();

    let mut roles: Vec<&String> = get_public_roles(&guild)
        .iter()
        .map(|role| &role.name)
        .collect();
    roles.sort();

    if roles.len() == 0 {
        print_or_log_error("No public roles.", &msg.channel_id);
        return Ok(());
    }
    
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
    print_or_log_error(&format!("```\n{}\n```", result), &msg.channel_id);
});

command!(joinrole(_ctx, msg, args) {
    let role_name = args.full();
    if role_name.is_empty() {
        print_or_log_error("Please enter a role name.", &msg.channel_id);
        return Ok(());
    }

    let guild = msg.guild().unwrap();
    let guildwritelock = guild.read();

    let role = get_public_roles(&guildwritelock)
        .into_iter()
        .find(|r| r.name.to_lowercase() == role_name);
    let role = match role {
        Some(r) => r,
        None => {
            print_or_log_error("Could not find role.", &msg.channel_id);
            return Ok(());
        }
    };

    let mut member = msg.member()
        .ok_or(format!("Could not get member from user {}", msg.author.id))?;

    if msg.author.has_role(guildwritelock.id, role) {
        member.remove_role(role)?;
        print_or_log_error("Role removed!", &msg.channel_id);
    } else {
        member.add_role(role)?;
        print_or_log_error("Role added!", &msg.channel_id);
    }
});

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

    // Skip until `vvv public vvv`
    roles
        .into_iter()
        .skip_while(|r| r.name != "vvv public vvv")
        .skip(1)
        .collect()
}
