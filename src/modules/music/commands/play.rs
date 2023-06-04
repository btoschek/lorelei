use std::time::Duration;

use super::super::events::{TrackEndNotifier, TrackStartNotifier};
use crate::modules::music::{status, TrackRequesterId};
use crate::modules::util::EmbedColor;
use crate::{edit_interaction_response, interaction_response};

use chrono::{NaiveDate, NaiveTime};
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
};

use songbird::{input::restartable::Restartable, TrackEvent};

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    react: bool,
) -> CommandResult {
    // Ensure bot is in the voice channel
    let _ = super::join::run(ctx, interaction, false).await;

    let url = interaction
        .data
        .options
        .get(0)
        .expect("Expected URL string")
        .resolved
        .as_ref()
        .expect("Valid UTF-8 String expected");

    let url = match url {
        CommandDataOptionValue::String(u) => u,
        _ => unreachable!("Non-string value in string parameter"),
    };

    if !url.starts_with("http") {
        return Ok(interaction_response!(interaction, ctx, |d| {
            d.ephemeral(true).embed(|e| {
                e.title("Invalid URL parameter")
                    .color(EmbedColor::Failure.hex())
            })
        }));
    }

    let guild_id = interaction.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        interaction_response!(interaction, ctx, |d| {
            d.ephemeral(true).embed(|e| {
                e.title("Searching ...")
                    .url(url)
                    .color(EmbedColor::Pending.hex())
            })
        });

        // Use lazy restartable sources to not pay
        // for decoding of tracks which aren't actually live yet
        let source = match Restartable::ytdl(url.clone(), true).await {
            Ok(source) => source,
            Err(_why) => {
                edit_interaction_response!(interaction, ctx, |d| {
                    d.embed(|e| e.title("Source not found").color(EmbedColor::Failure.hex()))
                });
                return Ok(());
            }
        };

        let track_handle = handler.enqueue_source(source.into());
        let mut typemap = track_handle.typemap().write().await;
        typemap.insert::<TrackRequesterId>(interaction.user.id);

        let _ = track_handle.add_event(
            songbird::Event::Delayed(Duration::new(0, 0)),
            TrackStartNotifier {
                ctx: ctx.clone(),
                queue: handler.queue().to_owned(),
            },
        );

        let _ = track_handle.add_event(
            songbird::Event::Track(TrackEvent::End),
            TrackEndNotifier {
                ctx: ctx.clone(),
                queue: handler.queue().to_owned(),
            },
        );

        let queue = handler.queue();
        if queue.len() > 1 {
            status::update_status(ctx, queue).await;
        }

        let meta = track_handle.metadata();

        if react {
            edit_interaction_response!(interaction, ctx, |d| {
                d.embed(|e| {
                    e.title(meta.title.as_ref().unwrap_or(&"No title".to_string()))
                        .url(meta.source_url.as_ref().unwrap())
                        .color(EmbedColor::Success.hex())
                        .thumbnail(
                            meta.thumbnail.as_ref().unwrap_or(
                                &"https://ak.picdn.net/shutterstock/videos/34370329/thumb/1.jpg"
                                    .to_string(),
                            ),
                        );

                    if let Some(artist) = &meta.artist.as_ref() {
                        e.footer(|a| a.text(artist));
                    }

                    if let Some(duration) = &meta.duration.as_ref() {
                        let time = NaiveTime::from_num_seconds_from_midnight_opt(
                            duration.as_secs() as u32,
                            0,
                        )
                        .expect("Just crash if someone is trolling with lengths exceeding the heat death of the universe");
                        e.field("Duration", time.format("%H:%M:%S"), true);
                    }

                    if let Some(date) = &meta.date.as_ref() {
                        let datetime = NaiveDate::parse_from_str(date, "%Y%m%d").expect("This format theoretically should not change");
                        e.field("Uploaded", datetime.format("%d.%m.%Y"), true);
                    }

                    e
                })
            });
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
        .description_localized("de", "Lasse mich einen bestimmten Audio Track spielen")
        .create_option(|option| {
            option
                .name("url")
                .description("The URL directing to your audio source")
                .description_localized("de", "Die URL zu deinem Audio Track")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
