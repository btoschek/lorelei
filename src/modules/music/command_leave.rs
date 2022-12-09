use serenity::client::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {

    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialization.").clone();

    if manager.get(guild_id).is_some() {

        if let Err(_e) = manager.remove(guild_id).await {
            // log_msg_err(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }
        // log_msg_err(msg.channel_id.say(&ctx.http, "Left voice channel").await);

    } else {
        // log_msg_err(msg.reply(ctx, "Not in a voice channel").await);
    }

    "OK".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .description("Leave current voice channel")
}
