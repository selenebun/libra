use crate::{utils, PermissionsContainer, StartTime};
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
#[commands(about, avatar, invite, ping, wikipedia, wiktionary)]
pub struct General;

#[command]
#[description("Get information about the bot.")]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    let cache = ctx.cache.read();
    let data = ctx.data.read();

    // Get the number of users in all guilds.
    let users = cache.guilds.values().fold(0, |acc, guild| {
        let guild = guild.read();
        acc + guild.member_count
    });

    let uptime = data
        .get::<StartTime>()
        .map(|t| t.elapsed().as_secs().to_string())
        .unwrap_or("N/A".to_string());

    let invite_link = invite_url(ctx)
        .map(|url| format!("\n[Invite me!]({})", url))
        .unwrap_or_else(|_| "".to_string());

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(Color::FOOYOO)
                .title(&cache.user.name)
                .thumbnail(cache.user.face())
                .description("I am a general purpose Discord bot, made with :heart: and Rust.")
                .field(
                    "Info",
                    format!(
                        "I am currently on {} servers, serving {} users in total.\nI have been online for {} seconds.",
                        cache.guilds.len(),
                        users,
                        uptime
                    ),
                    false,
                )
                .field("Links",
                    format!("[GitHub](https://github.com/rsaihe/libra){}", invite_link),
                    false,
                )
        })
    })?;

    Ok(())
}

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
#[description("Get the invite link for the bot.")]
fn invite(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("<{}>", invite_url(ctx)?))?;
    Ok(())
}

#[command]
#[description("Ping the bot.")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}

#[command]
#[aliases(w, wiki)]
#[description("Search a term on Wikipedia.")]
fn wikipedia(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(
        &ctx.http,
        format!(
            "<https://en.wikipedia.org/wiki/{}>",
            default_query(args.remains(), "Main_Page")
        ),
    )?;

    Ok(())
}

#[command]
#[aliases(wt)]
#[description("Search a term on Wiktionary.")]
fn wiktionary(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(
        &ctx.http,
        format!(
            "<https://en.wiktionary.org/wiki/{}>",
            default_query(args.remains(), "Wiktionary:Main_Page")
        ),
    )?;

    Ok(())
}

/// Return an optional query with the spaces converted to underscores, or
/// otherwise use a default.
fn default_query(query: Option<&str>, default: &str) -> String {
    query
        .map(|q| q.replace(' ', "_"))
        .unwrap_or_else(|| default.to_string())
}

/// Return the bot's invite URL with the appropriate permissions.
fn invite_url(ctx: &Context) -> serenity::Result<String> {
    let data = ctx.data.read();

    let perms = data
        .get::<PermissionsContainer>()
        .copied()
        .unwrap_or_else(|| Permissions::empty());

    ctx.cache.read().user.invite_url(&ctx.http, perms)
}
