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

mod suggestions;

use crate::{framework::commands::core::setup::suggestions::suggestions, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("suggestions"),
    category = "Core",
    owners_only,
    guild_only
)]
pub(crate) async fn setup(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}