use crate::interaction_response;

use serenity::{
    builder::CreateApplicationCommand, client::Context, framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Mentionable,
};

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    react: bool,
) -> CommandResult {
    let guild_id = interaction.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();
    let user_id = interaction.user.id;

    let channel_id = guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id);

    let channel_id = match channel_id {
        Some(channel) => channel,
        None => {
            // log_msg_err(msg.reply(ctx, "Not in a voice channel").await);
            return Ok(interaction_response!(interaction, ctx, |d| {
                d.content(":no_entry: Cannot find the voice channel you're in")
                    .ephemeral(true)
            }));
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, channel_id).await;

    if react {
        interaction_response!(interaction, ctx, |d| {
            d.content(format!(
                "Connected to voice channel {}",
                ctx.cache
                    .channel(channel_id)
                    .expect("User channel has to exist")
                    .mention(),
            ))
        });
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Join your voice channel")
}
