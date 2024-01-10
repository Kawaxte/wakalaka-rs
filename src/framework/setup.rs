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

use std::sync::atomic::AtomicUsize;

use tracing::{error, info, warn};

use crate::serenity::Context;
use crate::{modules, Data, Error};

pub(crate) async fn handle(ctx: &Context) -> Result<Data, Error> {
    register_guild_commands(ctx).await;

    Ok(Data {
        suggestion_id: AtomicUsize::new(1),
    })
}

async fn register_guild_commands(ctx: &Context) {
    let current_application_info = match ctx.http.get_current_application_info().await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get current application info");
            panic!("{why:?}");
        }
    };


    let guild_id = match current_application_info.guild_id {
        Some(value) => value,
        None => {
            warn!("No guild ID found in current application");
            return;
        }
    };
    let guild_name = {
        let guild = match ctx.cache.guild(guild_id) {
            Some(value) => value,
            None => {
                warn!("No guild found in cache");
                return;
            }
        };
        guild.name.clone()
    };

    let guild_commands = modules::guild_commands().await;

    let guild_command_count = guild_commands.len();
    //if none
    if guild_command_count == 0 {
        warn!("No guild command(s) to register in {guild_name}");
        return;
    }

    match poise::builtins::register_in_guild(&ctx.http, &guild_commands, guild_id).await {
        Ok(_) => {}
        Err(why) => {
            error!("Couldn't register guild commands");
            panic!("{why:?}");
        }
    }
    info!("Registered {guild_command_count} command(s) in {guild_name}");
}
