use serenity::model::id::MessageId;

use command_error::CommandError;

command!(purge(_ctx, msg, args) {
    // Get the number of messages to delete.
    let num = args.single::<u64>().unwrap();

    let channel = msg.channel()
        .ok_or(CommandError::Generic("Could not get channel.".to_owned()))?;
    let channel = channel.guild()
        .ok_or(CommandError::Generic("Could not get guild channel.".to_owned()))?;
    let channel = channel.write();
    let messages: Vec<MessageId> = channel.messages(|m| m.limit(num))
                                          .unwrap()
                                          .into_iter()
                                          .map(|m| m.id)
                                          .collect();

    channel.delete_messages(messages.into_iter())?;
});
