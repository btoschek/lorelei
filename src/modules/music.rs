use serenity::{client::Context, model::application::command::Command};

pub mod command_join;
pub mod command_leave;
pub mod command_play;

pub mod command {
    pub use super::command_join as join;
    pub use super::command_leave as leave;
    pub use super::command_play as play;
}

#[allow(unused_must_use)]
pub async fn register_commands(ctx: &Context) {
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
}
