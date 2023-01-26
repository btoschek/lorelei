use crate::interaction_response;

use serenity::{
    builder::CreateApplicationCommand, client::Context, framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    react: bool,
) -> CommandResult {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        if react {
            interaction_response!(interaction, ctx, |d| { d.content("Skipped current title") })
        }
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("skip")
        .description("Skip the current song")
        .description_localized("de", "Überspringe den aktuellen Titel")
}
