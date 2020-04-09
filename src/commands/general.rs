use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
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
#[commands(ping)]
pub struct General;

#[command]
#[description("Ping the bot.")]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}
