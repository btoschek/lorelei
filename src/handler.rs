use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
};
use tracing::{event, Level};

pub struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        event!(Level::INFO, "Connected as {}#{}",
              ready.user.name,
              ready.user.discriminator);
    }
}
