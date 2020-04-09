use serenity::cache::Cache;
use serenity::model::prelude::*;

/// Finds the UserId of a member matching a particular name in a guild.
pub fn find_user_in_guild<G: Into<GuildId>>(cache: &Cache, id: G, name: &str) -> Option<UserId> {
    cache
        .guild(id)
        .and_then(|guild| guild.read().member_named(name).map(|m| m.user_id()))
}
