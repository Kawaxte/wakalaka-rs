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

use crate::{
    database::infractions, framework::options::commands::moderator::InfractionType, Context, Error,
};

/// Removes warning from user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
pub(crate) async fn unwarn(
    ctx: Context<'_>,
    #[description = "Identifier (case) of warning to delete."] id: i32,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if id < 1 {
        let message = format!("Sorry, but you can't delete warnings with case ID less than 1.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let infraction_type = InfractionType::Warn.as_str();

    infractions::delete_infractions(id, infraction_type, pool).await;

    let message = format!("I've deleted warning with case ID `{}`.", id);
    let _ = ctx.reply(message).await;

    Ok(())
}