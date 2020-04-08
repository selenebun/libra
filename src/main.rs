use log::{error, info};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    // Load environment variables from .env file.
    if let Err(e) = kankyo::load(false) {
        eprintln!("Problem loading .env file: {}", e);
    }

    // Start logger.
    env_logger::builder().format_module_path(false).init();

    // Get token from environment.
    let token = match kankyo::key("DISCORD_TOKEN") {
        Some(token) => token,
        None => {
            error!("Expected a token in the environment");
            process::exit(1);
        }
    };

    // Log in with token.
    let mut client = match Client::new(&token, Handler) {
        Ok(client) => client,
        Err(e) => {
            error!("Problem logging in: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = client.start() {
        error!("Client error: {}", e);
        process::exit(1);
    };
}
