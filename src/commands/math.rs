use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(collatz)]
pub struct Math;

#[command]
#[description("Calculates the number of steps in the [Collatz sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) for a given positive integer.")]
#[num_args(1)]
#[usage("<number>")]
fn collatz(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut num = match args.parse::<u128>() {
        Ok(num) => num,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Unable to parse number.")?;
            return Ok(());
        }
    };

    // Calculate number of steps from repeatedly applying Collatz process.
    let mut steps = 0;
    while num > 1 {
        steps += 1;
        num = if num % 2 == 0 {
            num / 2
        } else {
            match num.checked_mul(3).and_then(|num| num.checked_add(1)) {
                Some(num) => num,
                None => {
                    msg.channel_id.say(
                        &ctx.http,
                        format!("Overflow occurred after {} iterations.", steps),
                    )?;
                    return Ok(());
                }
            }
        }
    }

    msg.channel_id
        .say(&ctx.http, format!("Finished after {} iterations.", steps))?;

    Ok(())
}
