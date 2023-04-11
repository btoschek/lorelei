use serenity::{
    client::Context,
    model::{application::command::Command, prelude::UserId},
    prelude::TypeMapKey,
};

pub mod command_join;
pub mod command_leave;
pub mod command_play;
pub mod command_skip;
mod events;
mod status;

pub mod command {
    pub use super::command_join as join;
    pub use super::command_leave as leave;
    pub use super::command_play as play;
    pub use super::command_skip as skip;
}


struct TrackRequesterId;

impl TypeMapKey for TrackRequesterId {
    type Value = UserId;
}

#[allow(unused_must_use)]
pub async fn register_commands(ctx: &Context) {
    status::ensure_channel_exists(ctx).await;

    Command::create_global_application_command(&ctx.http, |command| {
        self::command::join::register(command)
    })
    .await;

    Command::create_global_application_command(&ctx.http, |command| {
        self::command::leave::register(command)
    })
    .await;

    Command::create_global_application_command(&ctx.http, |command| {
        self::command::play::register(command)
    })
    .await;

    Command::create_global_application_command(&ctx.http, |command| {
        self::command::skip::register(command)
    })
    .await;
}
