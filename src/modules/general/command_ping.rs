use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(_ctx: &Context, _interaction: &ApplicationCommandInteraction) -> String {
    "Hey there !!!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Ping the bot to check latency")
}
