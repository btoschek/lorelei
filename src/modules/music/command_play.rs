use crate::interaction_response;

use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    framework::standard::CommandResult,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
        },
    },
    utils::Color,
};

use songbird::input::restartable::Restartable;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    react: bool,
) -> CommandResult {
    // Ensure bot is in the voice channel
    let _ = super::command::join::run(ctx, interaction, false).await;

    let url = interaction
        .data
        .options
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
        return Ok(interaction_response!(interaction, ctx, |d| {
            d.content("Invalid URL parameter")
        }));
    }

    let guild_id = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Use lazy restartable sources to not pay
        // for decoding of tracks which aren't actually live yet
        let source = match Restartable::ytdl(url.clone(), true).await {
            Ok(source) => source,
            Err(_why) => {
                // log_msg_err(msg.channel_id.say(&ctx.http, format!("{:?}", why)).await);
                return Ok(interaction_response!(interaction, ctx, |d| {
                    d.content("Wasn't able to get audio source")
                }));
            }
        };

        let track_handle = handler.enqueue_source(source.into());

        let meta = track_handle.metadata();

        //log_msg_err(msg.channel_id.say(&ctx.http, "Playing song").await);
        if react {
            interaction_response!(interaction, ctx, |d| {
                d.embed(|e| {
                    e.title(&meta.title.as_ref().unwrap_or(&"No title".to_string()))
                        .url(&meta.source_url.as_ref().unwrap())
                        .color(Color::new(0xb4f050))
                        .thumbnail(
                            &meta.thumbnail.as_ref().unwrap_or(
                                &"https://images6.alphacoders.com/766/thumb-1920-766470.png"
                                    .to_string(),
                            ),
                        );

                    if let Some(artist) = &meta.artist.as_ref() {
                        e.footer(|a| a.text(artist));
                    }

                    if let Some(duration) = &meta.duration.as_ref() {
                        let _ = duration.as_secs();
                    }

                    if let Some(date) = &meta.date.as_ref() {
                        let _ = date;
                    }

                    e
                })
            })
        }
    } else {
        //log_msg_err(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
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
