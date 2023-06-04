use serenity::{client::Context, model::prelude::UserId, prelude::TypeMapKey};

mod actions;
pub use actions::perform_action;
pub use actions::VoiceAction;

pub mod commands;
mod events;
mod status;

struct TrackRequesterId;

impl TypeMapKey for TrackRequesterId {
    type Value = UserId;
}

/// Initialize the module's functionality
pub async fn init(ctx: &Context) {
    status::ensure_channel_exists(ctx).await;
    commands::register(ctx).await;
}
