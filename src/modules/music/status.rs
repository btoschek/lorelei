use serenity::{
    client::Context,
    model::{
        id::GuildId,
        prelude::{ChannelType, PermissionOverwrite, PermissionOverwriteType, RoleId},
        Permissions,
    },
};
use std::env;

use crate::modules::util::EmbedColor;

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
