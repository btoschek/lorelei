use serenity::{
    model::prelude::interaction::message_component::MessageComponentInteraction, prelude::Context,
};

use super::status;

/// Toggle repeat status for the currently playing track
pub async fn current_track_set_repeat(ctx: &Context, interaction: &MessageComponentInteraction) {
    let guild_id = interaction.guild_id.expect("Can only be called in guilds");

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();
        let current = queue.current();

        if let Some(track) = current {
            let success = match interaction.data.custom_id.as_str() {
                "loop_on" => track.enable_loop(),
                "loop_off" => track.disable_loop(),
                _ => unreachable!("Further actions not supported"),
            };

            if success.is_ok() {
                status::set_currently_playing(ctx, queue).await;
            }
        }
    }
}

/// Start / stop playing the current track
pub async fn current_track_set_playing(ctx: &Context, interaction: &MessageComponentInteraction) {
    let guild_id = interaction.guild_id.expect("Can only be called in guilds");

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();
        let current = queue.current();

        if let Some(track) = current {
            let success = match interaction.data.custom_id.as_str() {
                "pause" => track.pause(),
                "resume" => track.play(),
                _ => unreachable!("Further actions not supported"),
            };

            if success.is_ok() {
                status::set_currently_playing(ctx, queue).await;
            }
        }
    }
}

/// Finish the playback of the currently playing song, starting the next one in the queue
pub async fn current_track_skip(ctx: &Context, interaction: &MessageComponentInteraction) {
    let guild_id = interaction.guild_id.expect("Can only be called in guilds");

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();
        let _ = queue.skip();

        // NOTE: We don't need to force an update here, because:
        //   - A new song automatically updates the status
        //   - If the skipped song were to be the last one,
        //     it automatically sets the status on track end
    }
}
