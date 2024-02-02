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

use tracing::error;

use crate::{utility::components::messages, Context};

pub(crate) async fn handle(ctx: Context<'_>) {
    let reply = messages::error_reply("Sorry, but I can only execute this command if invoked by 👑.", true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {:?}", why);
    }
}