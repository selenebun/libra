use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

#[group]
#[commands(eight_ball, shuffle)]
pub struct Fun;

#[command("8ball")]
#[description("Ask the Magic 8-Ball a question.")]
#[min_args(1)]
#[usage("question")]
fn eight_ball(ctx: &mut Context, msg: &Message) -> CommandResult {
    const ANSWERS: [&str; 20] = [
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes\u{2014}definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];

    let answer = ANSWERS.choose(&mut SmallRng::from_entropy()).unwrap();
    msg.channel_id.say(&ctx.http, answer)?;

    Ok(())
}

#[command]
#[aliases(scramble)]
#[description("Randomly scramble words.")]
#[min_args(1)]
#[usage("word...")]
fn shuffle(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    // Break up each Unicode word into individual graphemes and shuffle.
    let words: String = args
        .message()
        .split_word_bounds()
        .collect::<Vec<&str>>()
        .iter()
        .map(|word| {
            let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
            graphemes.shuffle(&mut SmallRng::from_entropy());
            graphemes.concat()
        })
        .collect();

    msg.channel_id.say(&ctx.http, &words)?;

    Ok(())
}
