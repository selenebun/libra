use crate::ShardManagerContainer;
use log::error;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process;

#[group]
#[commands(nickname, quit)]
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
