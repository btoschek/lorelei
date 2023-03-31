use serenity::{model::channel::Message, Result as SerenityResult};

/// Checks if a message was successfully sent. If not, logs why to stdout
pub fn log_msg_err(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

/// Consistent embed colors between commands
pub enum EmbedColor {
    Success,
    Failure,
    Pending,
}

impl EmbedColor {
    pub fn hex(&self) -> u32 {
        match self {
            EmbedColor::Success => 0xb4f050,
            EmbedColor::Failure => 0xff1a1a,
            EmbedColor::Pending => 0x1aa3ff,
        }
    }
}

/// Send response as reaction to original user interaction
#[macro_export]
macro_rules! interaction_response {
    ($interaction:ident, $ctx:ident, $builder:expr) => {
        $interaction
            .create_interaction_response(&$ctx, |response| {
                response.interaction_response_data($builder)
            })
            .await?
    };
}

/// Edit previously sent response for user interaction
#[macro_export]
macro_rules! edit_interaction_response {
    ($interaction:ident, $ctx:ident, $builder:expr) => {
        $interaction
            .edit_original_interaction_response(&$ctx, $builder)
            .await?
    };
}
