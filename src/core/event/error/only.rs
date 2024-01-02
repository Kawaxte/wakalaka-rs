/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub async fn on_not_an_owner(ctx: crate::Context<'_>) {
    let author = ctx.author();
    ctx.reply(format!("You're not my owner, {author}!"))
        .await
        .unwrap();
}

pub async fn on_guild_only(ctx: crate::Context<'_>) {
    let command = ctx.command();
    ctx.reply(format!("`{command:#?}` can only be used in a guild!"))
        .await
        .unwrap();
}

pub async fn on_dm_only(ctx: crate::Context<'_>) {
    let command = ctx.command();
    ctx.reply(format!(
        "`{command:#?}` can only be used in a Direct Message!"
    ))
    .await
    .unwrap();
}

pub async fn on_nsfw_only(ctx: crate::Context<'_>) {
    let command = ctx.command();
    ctx.reply(format!(
        "`{command:#?}` can only be used in a NSFW channel!"
    ))
    .await
    .unwrap();
}
