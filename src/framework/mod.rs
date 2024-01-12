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

use poise::{Framework, FrameworkOptions};
use tokio::time::Instant;
use tracing::info;

use crate::{Data, Error, commands};

pub(crate) mod options;
pub(crate) mod setup;

pub(crate) async fn initialise_framework(data: Data) -> Framework<Data, Error> {
    let start_time = Instant::now();

    let framework = Framework::builder()
    .setup(|ctx, _, _| {
        Box::pin(async move {
            setup::handle(ctx, data).await
        })
    })
    .options(FrameworkOptions {
        commands: commands::guild_commands().await,
        post_command: |ctx| Box::pin(options::post_command::handle(ctx)),
        event_handler: |ctx, event, framework, data| {
            Box::pin(options::event_handler::handle(
                ctx, event, framework, data,
            ))
        },
        ..Default::default()
    })
    .build();

    let elapsed_time = start_time.elapsed();
    info!("Initialised framework in {elapsed_time:.2?}");

    framework
}
