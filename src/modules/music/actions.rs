use serenity::{model::prelude::GuildId, prelude::Context};

use super::status;

pub enum VoiceAction {
    Play,
    Pause,
    LoopOn,
    LoopOff,
    Skip,
    Stop,
}

impl VoiceAction {
    pub fn from_str(action: &str) -> Option<Self> {
        match action {
            "play" => Some(Self::Play),
            "pause" => Some(Self::Pause),
            "loop_on" => Some(Self::LoopOn),
            "loop_off" => Some(Self::LoopOff),
            "skip" => Some(Self::Skip),
            "stop" => Some(Self::Stop),
            _ => None,
        }
    }
}

/// Interface with the underlying bot queue to manipulate the current track's state
pub async fn perform_action(ctx: &Context, guild_id: GuildId, action: VoiceAction) {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();
        let current = queue.current();

        if let Some(track) = current {
            let force_update = match action {
                VoiceAction::Play => track.play().is_ok(),
                VoiceAction::Pause => track.pause().is_ok(),
                VoiceAction::LoopOn => track.enable_loop().is_ok(),
                VoiceAction::LoopOff => track.disable_loop().is_ok(),
                VoiceAction::Skip => {
                    let _ = queue.skip();
                    false
                }
                VoiceAction::Stop => {
                    queue.stop();
                    false
                }
            };

            if force_update {
                status::set_currently_playing(ctx, queue).await;
            }
        }
    }
}
