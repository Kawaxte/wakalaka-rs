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

use tokio::time::Duration;
use tracing::{error, info};

use crate::{utility::messages, Context, Error};

/// Put yours truly to sleep.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    owners_only,
    guild_only
)]
pub(crate) async fn shutdown(
    ctx: Context<'_>,
    #[description = "Waiting time before sleep. (1-5s)"] duration: u64,
) -> Result<(), Error> {
    if duration < 1 || duration > 5 {
        let reply = messages::warn_reply("Duration must be between 1 and 5 seconds.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let reply = messages::reply(format!("Shutting down in {duration}s..."));
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
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
        info!("Shutting down shard {}", shard_id);
        manager.shutdown_finished(shard_id);

        tokio::time::sleep(Duration::from_secs(duration)).await;

        std::process::exit(0);
    }

    Ok(())
}
