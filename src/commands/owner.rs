use crate::ShardManagerContainer;
use log::error;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};
use std::process;

#[group]
#[commands(nickname, quit, servers)]
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
            error!("There was a problem accessing the shard manager");
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
