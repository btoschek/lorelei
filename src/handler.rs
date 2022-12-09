use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::Interaction,
        gateway::Ready,
    }
};
use tracing::{event, Level};
use crate::modules::general;
use crate::modules::music;

pub struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "ping" => general::command::ping::run(&ctx, &command).await,
                "join" => music::command::join::run(&ctx, &command).await,
                "leave" => music::command::leave::run(&ctx, &command).await,
                "play" => music::command::play::run(&ctx, &command).await,
                _ => "not implemented!".to_string(),
            };

            if let Err(_why) = command
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

        general::register_commands(&ctx).await;
        music::register_commands(&ctx).await;
    }
}
