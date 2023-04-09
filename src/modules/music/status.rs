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
