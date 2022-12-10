use serenity::client::Context;
use serenity::model::voice::VoiceState;
use serenity::model::channel::ChannelType;

/// Create a new voice channel when a user joins a specific voice channel.
/// Afterwards, move the user to the newly created channel.
pub async fn on_voice_channel_join(ctx: &Context, state: &VoiceState) {

    let guild = ctx.cache
        .guild(state.guild_id.expect("Voice channels are all in guilds"))
        .unwrap();

    let channel_id = state.channel_id.expect("VoiceState required when user in voice");
    let channel = ctx.cache
        .guild_channel(channel_id)
        .unwrap();

    let category_id = channel.parent_id.expect("All channels by design have one category they're in");
    let category = ctx.cache
        .category(category_id)
        .unwrap();

    if channel.name().starts_with('➕') {

        let mut name = category.name().to_string();
        let name = name.remove(0).to_uppercase().to_string() + &name;

        // Create new voice channel in same category
        let new_id = guild
            .create_channel(&ctx.http, |channel| {
                channel
                    .name(format!("🤖 {}", name))
                    .kind(ChannelType::Voice)
                    .category(category_id)
                    //.user_limit(channel.user_limit)
            }).await;

        // TODO: Check if channel was created

        // Move user to the new channel
        let _ = guild.move_member(&ctx.http,
            state.user_id,
            new_id.unwrap()
        ).await;

        // TODO: Allow channel creator to change things as max member count
    }
}

/// Cleanup unused voice channels previously created on-demand
pub async fn on_voice_channel_leave(ctx: &Context, state: &VoiceState) {

    let channel_id = state.channel_id.expect("Old VoiceState required when user leaves voice");
    let channel = ctx.cache
        .guild_channel(channel_id)
        .unwrap();

    let member_count = match channel.members(&ctx.cache).await {
        Ok(v) => v.len(),
        _ => return,
    };

    if member_count != 0 || !channel.name().starts_with('🤖') {
        return;
    }

    let _ = channel
        .delete(&ctx.http)
        .await;
}

pub async fn run(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {

    // User left voice chat
    if new.channel_id.is_none() {
        on_voice_channel_leave(ctx, old.as_ref().unwrap()).await;
        return;
    }

    // User switched channels
    if let Some(v) = old {
        on_voice_channel_leave(ctx, v).await;
    }

    on_voice_channel_join(ctx, new).await;
}
