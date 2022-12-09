use serenity::client::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {

    // Ensure bot is in the voice channel
    super::command::join::run(ctx, interaction).await;

    let url = interaction.data.options
        .get(0)
        .expect("Expected URL string")
        .resolved
        .as_ref()
        .expect("");

    let url = match url {
        CommandDataOptionValue::String(u) => u,
        _ => unreachable!("non-string value in string parameter"),
    };

    if !url.starts_with("http") {
        // log_msg_err(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return "Error".to_string();
    }

    let guild_id = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(_why) => {
                // log_msg_err(msg.channel_id.say(&ctx.http, format!("{:?}", why)).await);
                return "Error".to_string();
            },
        };

        handler.play_source(source);

        //log_msg_err(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        //log_msg_err(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    "OK".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play an audio stream from different sources")
        .create_option(|option| {
            option
                .name("url")
                .description("The URL directing to your audio source")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
