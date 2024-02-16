// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};

use crate::{utils::components, Context, Error};

#[poise::command(
    slash_command,
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Give your friend a hug.
pub(super) async fn hug(
    ctx: Context<'_>,
    #[description = "The user to hug."] user: User,
) -> Result<(), Error> {
    let user_id = user.id;

    let author = ctx.author();
    let author_id = author.id;
    if user_id == author_id {
        let reply = components::replies::error_reply_embed("Cannot 🫂 yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let user_mention = user.mention();

    let message = format!("🫂 {user_mention}");
    ctx.say(message).await?;

    Ok(())
}
