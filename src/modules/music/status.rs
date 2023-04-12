use chrono::{NaiveDate, NaiveTime};
use serenity::{
    client::Context,
    model::{
        id::GuildId,
        prelude::{
            component::ButtonStyle, ChannelId, ChannelType, Message, PermissionOverwrite,
            PermissionOverwriteType, ReactionType, RoleId,
        },
        Permissions,
    },
};
use songbird::tracks::{LoopState, PlayMode, TrackQueue};
use std::env;

use crate::modules::util::EmbedColor;

use super::TrackRequesterId;

const CHANNEL_NAME: &str = "🔊lorelei";

pub async fn ensure_channel_exists(ctx: &Context) {
    let guild_id = GuildId(env::var("TEST_GUILD").unwrap().parse().unwrap());
    let guild = ctx.cache.guild(guild_id).unwrap();

    let mut channel_exists = false;
    for (id, _) in guild.channels {
        let name = id.name(ctx).await;
        if name == Some(CHANNEL_NAME.to_string()) {
            channel_exists = true;
        }
    }

    if !channel_exists {
        let channel_handle = guild_id
            .create_channel(&ctx.http, |c| {
                c.name(CHANNEL_NAME)
                    .kind(ChannelType::Text)
                    .topic("Play your favorite bangers")
                    .permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_MESSAGES,
                        kind: PermissionOverwriteType::Role(RoleId(
                            *guild_id.as_u64(), // @everyone
                        )),
                    }])
            })
            .await;

        let bot = ctx.cache.current_user();
        let _ = channel_handle
            .unwrap()
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(EmbedColor::Success.hex())
                        .title(&bot.name)
                        .url("https://github.com/btoschek/lorelei")
                        .description("Play your favorite songs right in Discord")
                        .thumbnail(bot.face())
                        .field("Play a song", "/play URL", false)
                })
            })
            .await;
    }
}

/// Get a handle to the channel displaying bot state
async fn get_status_channel(ctx: &Context) -> Option<ChannelId> {
    let guild_id = GuildId(env::var("TEST_GUILD").unwrap().parse().unwrap());
    let guild = ctx.cache.guild(guild_id)?;

    for (id, _) in guild.channels {
        if id.name(ctx).await == Some(CHANNEL_NAME.to_string()) {
            return Some(id);
        }
    }

    None
}

/// Get a handle to the message used to convey bot information
pub async fn get_status_message(ctx: &Context) -> Option<Message> {
    let channel = get_status_channel(ctx).await?;

    let messages = channel
        .messages(&ctx.http, |retriever| retriever.limit(1))
        .await
        .ok()?;

    if messages.is_empty() {
        None
    } else {
        Some(messages.get(0).unwrap().to_owned())
    }
}

/// Set the status message to display information about the current track
pub async fn set_currently_playing(ctx: &Context, queue: &TrackQueue) {
    let mut message = get_status_message(ctx).await.unwrap();

    let current_track = queue.current();
    let current_track = match current_track {
        Some(track) => track,
        None => return,
    };

    let requested_user = current_track.typemap().read().await;
    let requested_user = requested_user.get::<TrackRequesterId>();
    let user = requested_user
        .unwrap()
        .to_user(&ctx)
        .await
        .expect("User has to exist");

    let meta = current_track.metadata();
    let state = current_track
        .get_info()
        .await
        .expect("TrackState should exist");

    let is_looping = !matches!(state.loops, LoopState::Finite(0));
    let is_playing = matches!(state.playing, PlayMode::Play);

    let _ = message
        .edit(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Listening to")
                    .description(format!(
                        "[{}]({})",
                        meta.title.as_ref().unwrap_or(&"Untitled".to_string()),
                        meta.source_url.as_ref().expect("We have to stream from something")
                    ))
                    .color(EmbedColor::Success.hex())
                    .thumbnail(
                        meta.thumbnail.as_ref().unwrap_or(
                            &"https://ak.picdn.net/shutterstock/videos/34370329/thumb/1.jpg"
                                .to_string(),
                        ),
                    );

                if let Some(artist) = &meta.artist.as_ref() {
                    e.footer(|f| f.text(artist));
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
                    let datetime = NaiveDate::parse_from_str(date, "%Y%m%d")
                        .expect("This format theoretically should not change");
                    e.field("Uploaded", datetime.format("%d.%m.%Y"), true);
                }

                if requested_user.is_some() {
                    e.field(
                        "Queued by",
                        format!("{}#{}", user.name, user.discriminator),
                        true,
                    );
                }

                if queue.len() > 1 {
                    e.field("Pending songs", queue.len() - 1, true);
                }

                e
            })
            .components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.emoji(ReactionType::Unicode("🔂".to_string()))
                            .style(if is_looping {
                                ButtonStyle::Success
                            } else {
                                ButtonStyle::Danger
                            })
                            .custom_id(if is_looping { "loop_off" } else { "loop_on" })
                    })
                    .create_button(|b| {
                        b.emoji(ReactionType::Unicode((if is_playing {"⏸"} else {"▶️"}).to_string()))
                            .style(ButtonStyle::Secondary)
                            .custom_id(if is_playing {"pause"} else {"play"})
                    })
                    .create_button(|b| {
                        b.emoji(ReactionType::Unicode("⏩".to_string()))
                            .style(ButtonStyle::Secondary)
                            .custom_id("skip")
                    })
                    .create_button(|b| {
                        b.emoji(ReactionType::Unicode("⏹".to_string()))
                            .style(ButtonStyle::Secondary)
                            .custom_id("stop")
                    })
                })
            })
        })
        .await;
}

/// Set the status message to its default idle state
pub async fn set_idle(ctx: &Context) {
    let mut message = get_status_message(ctx).await.unwrap();

    let bot = ctx.cache.current_user();

    let _ = message
        .edit(&ctx.http, |m| {
            m.embed(|e| {
                e.color(EmbedColor::Success.hex())
                    .title(&bot.name)
                    .url("https://github.com/btoschek/lorelei")
                    .description("Play your favorite songs right in Discord")
                    .thumbnail(bot.face())
                    .field("Play a song", "/play URL", false)
            })
            .components(|c| c)
        })
        .await;
}
