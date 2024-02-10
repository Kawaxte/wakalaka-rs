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

use chrono::Utc;
use serenity::{all::UserId, builder::EditMember};
use tracing::{error, info};

use crate::{
    database::{
        guild_members,
        infractions::{self, InfractionType},
        users,
    },
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "DEAFEN_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | DEAFEN_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Disallow a user from interaction in voice channels.
pub async fn deafen(
    ctx: Context<'_>,
    #[description = "The user to deafen."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for deafening."]
    #[min_length = 6]
    #[max_length = 80]
    reason: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await?;

    let moderator = models::users::author(ctx)?;
    let moderator_id = moderator.id;

    if user.bot || user.system {
        let reply =
            messages::error_reply("Sorry, but bots and system users cannot be deafened.", true);
        ctx.send(reply).await?;

        return Ok(());
    }
    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot deafen yourself.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let reason_char_count = reason.chars().count();
    if reason_char_count < 6 || reason_char_count > 80 {
        let reply =
            messages::info_reply("Reason must be between `6` and `80` characters long.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let result = {
        let (user_name, user_mention) =
            (&user.name, models::users::user_mention(ctx, user_id).await?);

        let (moderator_name, moderator_mention) =
            (&moderator.name, models::users::author_mention(ctx)?);

        let (guild_id, guild_name) = (
            models::guilds::guild_id(ctx)?,
            models::guilds::guild_name(ctx)?,
        );

        let created_at = Utc::now().naive_utc();

        let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;

        let mut member = models::members::member(ctx, guild_id, user_id).await?;
        let member_builder = EditMember::default().deafen(true);

        let message = messages::info_message(format!(
            "You've been deafened by {moderator_mention} in {guild_name} for {reason}.",
        ));
        user.direct_message(ctx, message).await?;

        match member.edit(ctx, member_builder).await {
            Ok(_) => {
                guild_members::update_guilds_members_set_deaf(&user_id, true, pool).await?;

                infractions::insert_into_infractions(
                    InfractionType::Deaf,
                    &user_id,
                    &moderator_id,
                    &reason,
                    created_at,
                    &guild_id,
                    pool,
                )
                .await?;

                user_infractions += 1;

                users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

                info!("@{moderator_name} deafened @{user_name} in {guild_name}: {reason}");
                Ok(format!("{user_mention} has been deafened."))
            }
            Err(why) => {
                error!("Couldn't deafen @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't deafen {user_mention}."))
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
