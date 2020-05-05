use log::{error, info, warn};
use serde::de;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::process;
use std::sync::Arc;
use std::time::Instant;

mod commands;
mod utils;

use commands::{FUN_GROUP, GENERAL_GROUP, HELP, MATH_GROUP, OWNER_GROUP};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = String;
}

struct PermissionsContainer;

impl TypeMapKey for PermissionsContainer {
    type Value = Permissions;
}

struct Prefixes;

impl TypeMapKey for Prefixes {
    type Value = Arc<RwLock<HashMap<GuildId, String>>>;
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct StartTime;

impl TypeMapKey for StartTime {
    type Value = Instant;
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

    // Allow data to be shared across shards.
    {
        let mut data = client.data.write();
        data.insert::<DefaultPrefix>(match kankyo::key("PREFIX") {
            Some(prefix) => prefix,
            None => {
                error!("Expected a default bot prefix in the environment");
                process::exit(1);
            }
        });
        data.insert::<PermissionsContainer>(
            match kankyo::key("PERMS").and_then(|p| p.parse().ok()) {
                Some(p) => Permissions::from_bits_truncate(p),
                None => Permissions::empty(),
            },
        );
        data.insert::<Prefixes>(Arc::new(RwLock::new({
            match kankyo::key("PREFIX_FILE") {
                Some(file) => fs::read_to_string(file)
                    .map_err(de::Error::custom)
                    .and_then(|contents| serde_json::from_str(&contents))
                    .unwrap_or_else(|e| {
                        warn!("Problem reading prefix file: {:?}", e);
                        HashMap::new()
                    }),
                None => {
                    error!("Expected a prefix file in the environment");
                    process::exit(1);
                }
            }
        })));
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<StartTime>(Instant::now());
    }

    // Get owner.
    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);
            set
        }
        Err(e) => {
            error!("Problem accessing application info: {}", e);
            process::exit(1);
        }
    };

    // Configure command framework.
    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.owners(owners).dynamic_prefix(|ctx, msg| {
                    let data = ctx.data.read();
                    match data.get::<Prefixes>() {
                        Some(prefixes) => msg
                            .guild_id
                            .and_then(|id| prefixes.read().get(&id).cloned())
                            .or_else(|| {
                                data.get::<DefaultPrefix>().cloned().or_else(|| {
                                    error!("Problem accessing default prefix");
                                    process::exit(1);
                                })
                            }),
                        None => {
                            error!("Problem accessing server prefixes");
                            process::exit(1);
                        }
                    }
                })
            })
            .group(&GENERAL_GROUP)
            .group(&FUN_GROUP)
            .group(&MATH_GROUP)
            .group(&OWNER_GROUP)
            .help(&HELP)
            .after(|_, _, command, result| {
                if let Err(e) = result {
                    warn!("Problem in {} command: {:?}", command, e);
                }
            }),
    );

    // Run the client.
    let result = client.start();

    // Write server-local prefixes to a file.
    {
        let data = client.data.read();
        if let Some(prefixes) = data.get::<Prefixes>() {
            match kankyo::key("PREFIX_FILE") {
                Some(file) => {
                    match fs::write(file, serde_json::to_string(&*prefixes.read()).unwrap()) {
                        Ok(_) => info!("Prefix file successfully written"),
                        Err(e) => error!("Problem writing prefix file: {:?}", e),
                    }
                }
                None => error!("Expected a prefix file in the environment"),
            }
        }
    }

    if let Err(e) = result {
        error!("Client error: {}", e);
        process::exit(1);
    };
}
