use serenity::{client::Context, model::application::command::Command};

pub mod join;
pub mod leave;
pub mod play;
pub mod skip;

#[allow(unused_must_use)]
pub async fn register(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| join::register(command)).await;
    Command::create_global_application_command(&ctx.http, |command| leave::register(command)).await;
    Command::create_global_application_command(&ctx.http, |command| play::register(command)).await;
    Command::create_global_application_command(&ctx.http, |command| skip::register(command)).await;
}
