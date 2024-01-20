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

use tracing::{error, info};

use crate::{utility::messages, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    owners_only,
    guild_only
)]
/// Restart yours truly to her former glory.
pub(crate) async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    let reply = messages::reply("Restarting yours truly...", true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(Error::from(why));
    }

    let manager = ctx.framework().shard_manager.clone();

    let shard_ids = manager
        .runners
        .lock()
        .await
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    for shard_id in shard_ids {
        info!("Restarting shard {}", shard_id);
        manager.restart(shard_id).await;
    }

    Ok(())
}
