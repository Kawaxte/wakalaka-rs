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

use poise::{Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions};
use serenity::all::GatewayIntents;
use tokio::time::Instant;
use tracing::info;

use crate::framework::options::commands;
use crate::{Data, Error};

pub(super) mod options;
pub(super) mod setup;

pub(crate) async fn initialise_framework(data: Data) -> Framework<Data, Error> {
    let start_time = Instant::now();

    let framework = Framework::builder()
        .setup(|ctx, _, _| Box::pin(async move { setup::handle(ctx, data).await }))
        .options(FrameworkOptions {
            commands: commands::guild_commands().await,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(format!("?")),
                additional_prefixes: vec![Prefix::Literal("::")], // Inspired by RuneScape's command prefix.
                ..Default::default()
            },
            post_command: |ctx| Box::pin(options::post_command::handle(ctx)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(options::event_handler::handle(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let elapsed_time = start_time.elapsed();
    info!("Initialised framework in {elapsed_time:.2?}");

    framework
}

pub(crate) fn initialise_intents() -> GatewayIntents {
    let start_time = Instant::now();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let elapsed_time = start_time.elapsed();
    info!("Initialised intents in {elapsed_time:.2?}");

    intents
}
