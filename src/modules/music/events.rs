use serenity::{async_trait, prelude::Context};
use songbird::{tracks::TrackQueue, Event, EventContext, EventHandler as VoiceEventHandler};

use super::status;

pub struct TrackEndNotifier {
    pub ctx: Context,
    pub queue: TrackQueue,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if self.queue.is_empty() {
            status::set_idle(&self.ctx).await;
        }
        None
    }
}

pub struct TrackStartNotifier {
    pub ctx: Context,
    pub queue: TrackQueue,
}

#[async_trait]
impl VoiceEventHandler for TrackStartNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        status::set_currently_playing(&self.ctx, &self.queue).await;
        None
    }
}
