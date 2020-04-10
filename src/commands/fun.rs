use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

#[group]
#[commands(shuffle)]
pub struct Fun;

#[command]
#[aliases(scramble)]
#[description("Randomly scramble words.")]
#[min_args(1)]
#[usage("word...")]
fn shuffle(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut rng = SmallRng::from_entropy();

    // Break up each Unicode word into individual graphemes and shuffle.
    let words: String = args
        .message()
        .split_word_bounds()
        .collect::<Vec<&str>>()
        .iter()
        .map(|word| {
            let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
            graphemes.shuffle(&mut rng);
            graphemes.concat()
        })
        .collect();

    msg.channel_id.say(&ctx.http, &words)?;

    Ok(())
}
