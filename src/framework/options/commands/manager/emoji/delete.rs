// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use tracing::{error, info, warn};

use crate::{
    utility::{
        components::{self, messages},
        models,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete an existing emoji.
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
) -> Result<(), Error> {
    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let emoji_id = match components::emojis::emoji_id(ctx, &name).await {
        Some(emoji_id) => emoji_id,
        None => {
            warn!("Couldn't find {name:?} emoji in {guild_name}");

            let reply =
                messages::warn_reply(format!("Emoji called `{name}`. doesn't exist!"), true);
            ctx.send(reply).await?;

            return Ok(());
        }
    };

    let emoji = components::emojis::emoji(ctx, emoji_id).await.unwrap();
    let emoji_name = &emoji.name;

    let result = match guild.delete_emoji(ctx, emoji_id).await {
        Ok(_) => {
            let user_name = &ctx.author().name;

            info!("@{user_name} deleted {emoji_name:?} emoji from {guild_name}");
            Ok(format!("Deleted an emoji called `{emoji_name}`."))
        }
        Err(why) => {
            error!("Failed to delete {emoji_name:?} emoji from {guild_name}: {why:?}");
            Err(format!(
                "An error occurred whilst deleting an emoji called `{emoji_name}`"
            ))
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
