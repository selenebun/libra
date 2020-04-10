[![](https://img.shields.io/github/workflow/status/rsaihe/libra/Rust?style=flat-square)](https://github.com/rsaihe/libra/actions?query=workflow%3ARust)
[![](https://img.shields.io/crates/v/libra-bot.svg?style=flat-square)](https://crates.io/crates/libra-bot)
[![](https://img.shields.io/github/license/rsaihe/libra?style=flat-square)](LICENSE)

# libra

A general-purpose Discord bot made with the
[Serenity] library.

## Setup

Once you clone the repository, copy the [`.env.example`](.env.example) file and
rename it to `.env`.

After creating a new bot application on Discord's [developer portal], copy the
token to the end of the `DISCORD_TOKEN=` line of the `.env` file.

Assuming Rust is installed, you should be able to run the bot with `cargo run
--release`.

## How to invite to a server

You can find your application's client ID on the [developer portal]. Once
you have it, simply use the following link to invite the bot to a server:

<https://discordapp.com/api/oauth2/authorize?client_id=CLIENT_ID&scope=bot&permissions=67488832>

Note that you will have to replace the `CLIENT_ID` in the URL with the ID you
copied.

[developer portal]: https://discordapp.com/developers/applications
[Serenity]: https://github.com/serenity-rs/serenity
