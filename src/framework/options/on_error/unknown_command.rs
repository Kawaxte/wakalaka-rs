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

use serenity::all::Message;
use tracing::error;

use crate::utility::components::messages;

pub(crate) async fn handle(ctx: &crate::serenity::Context, msg: &Message, msg_content: String) {
    let channel_id = msg.channel_id;

    let message = messages::error_message(format!("`{msg_content}` isn't a valid command."));
    if let Err(why) = channel_id.send_message(&ctx, message).await {
        error!("Failed to send message: {why:?}");
    }
}
