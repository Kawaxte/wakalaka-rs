// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::{utils::builders, Context};

pub(crate) async fn handle(ctx: Context<'_>) {
    let command = &ctx.command();
    let command_name = &command.name;

    let reply = builders::replies::error_reply_embed(
        format!("Cannot invoke `{command_name}` without ownership of yours truly."),
        true,
    );

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
