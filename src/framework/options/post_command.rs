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

use tracing::info;

use crate::Context;

pub async fn handle(ctx: Context<'_>) {
    let (user_name, command_name, channel_name) = (
        &ctx.author().name,
        &ctx.command().qualified_name,
        ctx.channel_id()
            .name(ctx)
            .await
            .expect("No channel name found"),
    );
    
    info!("@{user_name} invoked {command_name:?} in #{channel_name}");
}
