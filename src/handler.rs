use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::Ready,
    }
};
use tracing::{event, Level};
use crate::modules::general;

pub struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "ping" => general::command::ping::run(&command.data.options),
                _ => "not implemented!".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                todo!("Implement");
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {

        event!(Level::INFO, "Connected as {}#{}",
              ready.user.name,
              ready.user.discriminator);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            crate::modules::general::command::ping::register(command)
        })
        .await;
    }
}
