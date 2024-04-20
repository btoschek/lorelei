use std::time::Duration;

use super::super::events::{TrackEndNotifier, TrackStartNotifier};
use crate::modules::music::{providers::ytdlp::YtDlp, status, TrackRequesterId};
use crate::modules::util::EmbedColor;
use crate::{edit_interaction_response, interaction_response};

use serenity::model::id::UserId;
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
use songbird::Call;
use tokio::sync::MutexGuard;

use songbird::TrackEvent;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    react: bool,
) -> CommandResult {
    // Ensure bot is in the voice channel
    // FIXME: This was super bodged to begin with, don't ever do this shit, refactor asap
    let _ = super::join::run(ctx, interaction, false).await;

    let url = interaction
        .data
        .options
        .first()
        .expect("Expected URL string")
        .resolved
        .as_ref()
        .expect("Valid UTF-8 String expected");

    let CommandDataOptionValue::String(url) = url else {
        unreachable!("Non-string value in String parameter")
    };

    // TODO: Use search instead
    if !url.starts_with("http") {
        return Ok(interaction_response!(interaction, ctx, |d| {
            d.ephemeral(true).embed(|e| {
                e.title("Invalid URL parameter")
                    .color(EmbedColor::Failure.hex())
            })
        }));
    }

    let guild_id = interaction
        .guild_id
        .expect("GuildId has to be set in Guilds");
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // Return early if no Call (voice connection manager) exists for the current guild
    // Tip: If no Call exists, you should make sure to let the bot join a voice channel before trying again
    // TODO: This could be expected here, due to forcing the bot to join a voice channel above
    let call = manager.get(guild_id);
    if call.is_none() {
        interaction_response!(interaction, ctx, |d| {
            d.ephemeral(true).embed(|e| {
                e.title("Not in a voice channel")
                    .description("Please connect the bot to a voice channel first")
                    .color(EmbedColor::Failure.hex())
            })
        });
        return Err("Not in a voice channel to play audio in".into());
    }

    let handler_lock = call.expect("call.is_none() already checked above");
    let mut handler = handler_lock.lock().await;

    interaction_response!(interaction, ctx, |d| {
        d.ephemeral(true).embed(|e| {
            e.title("Searching ...")
                .url(url)
                .color(EmbedColor::Pending.hex())
        })
    });

    let urls_to_queue: Vec<String> = {
        if url.contains("/playlist?list=") {
            YtDlp::playlist(url.clone())
                .into_iter()
                .map(|v| v.url)
                .collect()
        } else {
            vec![url.to_string()]
        }
    };

    edit_interaction_response!(interaction, ctx, |d| {
        d.embed(|e| {
            e.title(format!(
                "Adding {} songs to the queue ...",
                urls_to_queue.len()
            ))
            .url(url)
            .color(EmbedColor::Pending.hex())
        })
    });

    // Actually queue tracks here
    for url in urls_to_queue {
        enqueue_track(ctx, url, &mut handler, interaction.user.id).await;
    }

    let queue = handler.queue();
    if queue.len() > 1 {
        status::update_status(ctx, queue).await;
    }

    // Respond to the user with a confirmation message
    edit_interaction_response!(interaction, ctx, |d| {
        d.embed(|e| {
            e.title("Added all songs to the queue")
                .color(EmbedColor::Success.hex())
        })
    });

    Ok(())
}

async fn enqueue_track(
    ctx: &Context,
    url: String,
    handler: &mut MutexGuard<'_, Call>,
    user_id: UserId,
) {
    // TODO: Don't unwrap here, propagate error upwards
    let source = YtDlp::url(url).await.unwrap();

    let track_handle = handler.enqueue_source(source.into());
    let mut typemap = track_handle.typemap().write().await;
    typemap.insert::<TrackRequesterId>(user_id);

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
