use crate::modules::{auto, general, music};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::interaction::Interaction, gateway::Ready, voice::VoiceState},
};
use tracing::{event, Level};

pub struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {
    /// Handle any slash command interactions with a user
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let _ = match command.data.name.as_str() {
                // "ping" => general::command::ping::run(&ctx, &command).await,
                "join" => music::command::join::run(&ctx, &command, true).await,
                "leave" => music::command::leave::run(&ctx, &command, true).await,
                "play" => music::command::play::run(&ctx, &command, true).await,
                "skip" => music::command::skip::run(&ctx, &command, true).await,
                _ => unreachable!("No further commands implemented"),
            };
        }

        if let Interaction::MessageComponent(component) = interaction {
            let _ = match component.data.custom_id.as_str() {
                "loop_on" | "loop_off" => {
                    music::action::current_track_set_repeat(&ctx, &component).await
                }
                "pause" => todo!("Implement"),
                "skip" => todo!("Implement"),
                "stop" => todo!("Implement"),
                _ => unreachable!("No further actions implemented"),
            };

            let _ = component.defer(ctx).await;
        }
    }

    /// Signal to Discord the commands our bot exposes
    async fn ready(&self, ctx: Context, ready: Ready) {
        event!(
            Level::INFO,
            "Connected as {}#{}",
            ready.user.name,
            ready.user.discriminator
        );

        general::register_commands(&ctx).await;
        music::register_commands(&ctx).await;
    }

    /// Trigger automated voice-chat related functionalities
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        auto::dynamic_voice_channels::run(&ctx, &old, &new).await;
    }
}
