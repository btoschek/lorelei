use crate::interaction_response;

use serenity::{
    builder::CreateApplicationCommand, client::Context, framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _react: bool,
) -> CommandResult {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if manager.get(guild_id).is_some() {
        if let Err(_e) = manager.remove(guild_id).await {
            // log_msg_err(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        } else {
            interaction_response!(interaction, ctx, |d| { d.content("Disconnected") })
        }
        // log_msg_err(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        // log_msg_err(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .description("Leave current voice channel")
        .description_localized("de", "Lass mich den aktuellen Sprachkanal verlassen")
}
