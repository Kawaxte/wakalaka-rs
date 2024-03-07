// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod server;
mod user;

use crate::{
    framework::options::commands::core::unrestrict::{server::server, user::user},
    Context, Error,
};

#[poise::command(
    slash_command,
    subcommands("server", "user"),
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn unrestrict(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
