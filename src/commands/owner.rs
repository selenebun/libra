use crate::{DefaultPrefix, Prefixes, ShardManagerContainer};
use log::error;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};
use std::process;

#[group]
#[commands(nickname, prefix, quit, servers)]
#[owners_only]
pub struct Owner;

#[command]
#[description(
    "Edit the bot's nickname on a server. Resets the nickname if no arguments are provided."
)]
#[only_in(guilds)]
#[usage("[name]")]
fn nickname(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    // Reset the nickname if no args were provided.
    let name = if args.is_empty() {
        None
    } else {
        Some(args.message())
    };

    let guild = msg.guild_id.unwrap();
    guild.edit_nickname(&ctx.http, name)?;

    Ok(())
}

#[command]
#[description(
    "Change the bot's prefix on the current server. Resets if no arguments are provided."
)]
#[only_in(guilds)]
#[usage("[prefix]")]
fn prefix(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    match data.get::<Prefixes>() {
        Some(prefixes) => {
            let mut prefixes = prefixes.write();
            if args.is_empty() {
                msg.channel_id.say(
                    &ctx.http,
                    format!(
                        "Reset prefix to `{}`.",
                        data.get::<DefaultPrefix>().unwrap_or_else(|| {
                            error!("Expected a default bot prefix in the environment");
                            process::exit(1);
                        })
                    ),
                )?;
                prefixes.remove(&msg.guild_id.unwrap());
            } else {
                msg.channel_id.say(
                    &ctx.http,
                    format!("Changed prefix to `{}`.", args.message()),
                )?;
                prefixes.insert(msg.guild_id.unwrap(), args.message().to_string());
            }
        }
        None => {
            error!("Problem accessing prefixes");
            process::exit(1);
        }
    }
    Ok(())
}

#[command]
#[description("Shut down the bot.")]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    ctx.invisible();

    let data = ctx.data.read();
    match data.get::<ShardManagerContainer>() {
        Some(manager) => {
            let _ = msg.channel_id.say(&ctx.http, "Shutting down!");
            manager.lock().shutdown_all();
        }
        None => {
            error!("Problem accessing the shard manager");
            process::exit(1);
        }
    }

    Ok(())
}

#[command]
#[aliases(guilds)]
#[description("List all the servers the bot is currently in.")]
fn servers(ctx: &mut Context, msg: &Message) -> CommandResult {
    let cache = ctx.cache.read();

    // Get a vector of server names.
    let mut names: Vec<_> = cache
        .guilds
        .values()
        .map(|guild| guild.read().name.clone())
        .collect();
    names.sort_unstable();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO).title("Servers").description({
                let mut content = MessageBuilder::new();
                for name in &names {
                    content.push_line(name);
                }
                content.build()
            })
        })
    })?;

    Ok(())
}
