use std::collections::HashSet;
use serenity::{
    framework::standard::{
        Args,
        CommandGroup,
        CommandResult,
        help_commands,
        HelpOptions,
        macros::help,
    },
    model::{
        id::UserId,
        channel::Message,
    },
    prelude::*,
};

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
