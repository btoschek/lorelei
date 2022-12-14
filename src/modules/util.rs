use serenity::{model::channel::Message, Result as SerenityResult};

/// Checks if a message was successfully sent. If not, logs why to stdout
pub fn log_msg_err(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
