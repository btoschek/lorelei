use serenity::client::Context;
use serenity::model::application::command::Command;

pub mod command_ping;

pub mod command {
    pub use super::command_ping as ping;
}

#[allow(unused_must_use)]
pub async fn register_commands(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        self::command::ping::register(command)
    }).await;
}
