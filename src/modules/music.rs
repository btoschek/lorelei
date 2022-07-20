use serenity::{
    client::Context,
    framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    },
    model::channel::Message,
};
use super::util::log_msg_err;

#[group]
#[only_in(guilds)]
#[commands(join, leave, play, playlist)]
#[description("Manage the playback of audio tracks from different audio sources")]
struct Music;

#[command]
#[description("Join the caller's voice channel")]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            log_msg_err(msg.reply(ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[description("Leave the current voice channel")]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            log_msg_err(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        log_msg_err(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        log_msg_err(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[description("Play a song from a variety of different audio sources")]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Ensure bot is in the voice channel
    join(ctx, msg, args.clone()).await?;

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            log_msg_err(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        log_msg_err(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                log_msg_err(msg.channel_id.say(&ctx.http, format!("{:?}", why)).await);
                return Ok(());
            },
        };

        handler.play_source(source);

        log_msg_err(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        log_msg_err(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}
