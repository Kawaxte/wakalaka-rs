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

use super::Post;
use crate::{commands, Context};
use serenity::{
    all::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage,
    },
};

const DANBOORU_URL: &str = "https://danbooru.donmai.us";
const DANBOORU_LOGO_PNG: &str =
    "https://danbooru.donmai.us/packs/static/images/danbooru-logo-128x128-ea111b6658173e847734.png";

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>],
) -> Option<String> {
    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "post" => post(ctx, options, interaction).await,
        _ => None,
    }
}

async fn id(options: &[ResolvedOption<'_>]) -> Option<String> {
    let option = options.get(0)?;
    if option.name == "post" {
        if let ResolvedValue::SubCommand(subcommand) = &option.value {
            let suboption = subcommand.get(0)?;
            if suboption.name == "id" {
                if let ResolvedValue::Integer(id) = &suboption.value {
                    return Some(id.to_string());
                }
            }
        }
    }
    None
}

async fn post(
    ctx: &Context,
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> Option<String> {
    let id = id(options)
        .await
        .unwrap_or_default()
        .parse::<i64>()
        .expect("Error while parsing ID");
    let channel_id = interaction.channel_id;

    let exists = Post::exists(ctx, channel_id, id).await;
    if exists {
        let posts_json = format!("{DANBOORU_URL}/posts/{id}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&posts_json)
            .send()
            .await
            .expect("Error while sending get request")
            .text()
            .await
            .expect("Error while getting text from response");
        let response_json =
            serde_json::from_str(&response_text).expect("Error while parsing JSON from response");

        let success = Post::has_success(ctx, &response_json, channel_id, id).await;
        if success {
            let post = Post::new(&response_json);

            let embed = Post::embed(
                &post,
                DANBOORU_LOGO_PNG,
                id,
                None,
                DANBOORU_URL,
                post.file_url.clone(),
                Post::generate_footer(&post),
                0xAC8A64,
            );

            let response_message = CreateInteractionResponseMessage::default();

            let message = response_message.add_embed(embed);
            let response = CreateInteractionResponse::Message(message);

            let _ = interaction.create_response(&ctx.http, response).await;
        }
    }
    None
}

pub fn register() -> CreateCommand {
    CreateCommand::new("danbooru")
        .description(format!("Provides interactibility with {DANBOORU_URL:?}").as_str())
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "post",
                "Previews a post from Danbooru",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::Integer, "id", "Index of the post.")
                    .required(true),
            ),
        )
}
