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

use ::serenity::builder::CreateEmbed;
use poise::CreateReply;
use serenity::{
    all::colours::branding,
    builder::{CreateEmbedAuthor, CreateEmbedFooter},
    model::Colour,
};

use crate::{Context, Error};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

const GITHUB_URL: &str = "https://github.com/Kawaxte";

/// Fetches basic information about yours truly.
#[poise::command(slash_command)]
pub(crate) async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let bot = match ctx.http().get_current_user().await {
        Ok(value) => value,
        Err(why) => {
            return Err(format!("Couldn't get information of current user: {why:?}").into());
        }
    };
    let bot_avatar_url = match bot.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => bot.default_avatar_url(),
    };

    let embed = embed(&bot_avatar_url, branding::BLURPLE);
    let reply = CreateReply {
        content: None,
        embeds: vec![embed],
        ..Default::default()
    };
    let _ = ctx.send(reply).await;

    Ok(())
}

fn embed(icon_url: &String, colour: Colour) -> CreateEmbed {
    CreateEmbed::default()
        .author(embed_author(icon_url))
        .title(format!("{} v{}", NAME, VERSION))
        .description(DESCRIPTION)
        .url(format!("{GITHUB_URL}/{NAME}"))
        .footer(embed_footer())
        .colour(colour)
}

fn embed_footer() -> CreateEmbedFooter {
    let footer_text = format!("Powered by Rust {}", RUST_VERSION);

    CreateEmbedFooter::new(footer_text)
}

fn embed_author(icon_url: &String) -> CreateEmbedAuthor {
    let author = match AUTHORS.split(',').next() {
        Some(value) => value,
        None => "No author found",
    };

    CreateEmbedAuthor::new(author).icon_url(icon_url)
}
