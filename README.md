# libra

A Discord bot made with the [Serenity](https://github.com/serenity-rs/serenity)
library.

## Setup

Once you clone the repository, copy the [`.env.example`](.env.example) file and
rename it to `.env`.

After creating a new bot application on Discord's developer portal, copy the
token to the end of the `DISCORD_TOKEN=` line of the `.env` file.

Assuming Rust is installed, you should be able to run the bot with `cargo run
--release`.
