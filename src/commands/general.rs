use crate::utils;
use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::collections::HashSet;

#[help]
#[dm_and_guild_text("In DMs and servers")]
#[embed_error_colour(RED)]
#[embed_success_colour(FOOYOO)]
#[guild_only_text("Only in servers")]
#[lacking_ownership(hide)]
#[lacking_permissions(hide)]
#[lacking_role(hide)]
#[max_levenshtein_distance(2)]
#[strikethrough_commands_tip_in_dm(false)]
#[strikethrough_commands_tip_in_guild(false)]
pub fn help(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

#[group]
#[commands(avatar, ping)]
pub struct General;

#[command]
#[description("Get a user's avatar. Gets your own avatar if no user is provided.")]
fn avatar(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let user = if args.is_empty() {
        Some(msg.author.clone())
    } else {
        // If arguments are provided, try to find a matching user.
        msg.guild_id
            .and_then(|id| utils::find_user_in_guild(&ctx.cache.read(), id, args.message()))
            .and_then(|id| id.to_user(&ctx.http).ok())
    };

    let user = match &user {
        Some(user) => user,
        None => {
            msg.channel_id.say(&ctx.http, "User not found.")?;
            return Ok(());
        }
    };

    // Get the URL for the user's avatar.
    let mut url = user.face();
    let idx = url.find('?').unwrap_or(url.len());
    url.truncate(idx);

    // Get the nickname of the user if in a guild.
    let name = msg.guild_id.and_then(|id| user.nick_in(&ctx.http, id));

    // Send an embed containing the user's name and the avatar.
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO)
                .image(url)
                .title(name.as_ref().unwrap_or_else(|| &user.name))
        })
    })?;

    Ok(())
}

#[command]
#[description("Ping the bot.")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}
