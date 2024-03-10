// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod artist;

use crate::{framework::options::commands::music::lastfm::artist::artist, Context, Throwable};

const LASTFM_COLOUR: u32 = 0xA90000;

const MUSIC_URL: &str = "https://www.last.fm/music/";

#[poise::command(
    slash_command,
    subcommands("artist"),
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn lastfm(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
