use crate::api::ChannelInfoBuilder;
use crate::ApiClient;
use crate::{exit, matches_any, warning};
use figment::{
    providers::{Env, Format, Toml, Yaml},
    Figment,
};
use futures::future::JoinAll;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use twitch_api2::helix::points::{CustomReward, UpdateCustomRewardBody};

#[derive(Deserialize, Debug)]
struct Config {
    config_locale: Option<String>,
    tags: Option<Vec<String>>,
    language: Option<String>,
    title: Option<String>,
    category: Option<String>,
    notification: Option<String>,
    // #[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
    rewards: Option<Vec<String>>,
}

pub fn valid_extension(file: &PathBuf) -> bool {
    if let Some(e) = file.extension() {
        matches_any!(e, "ini", "toml", "yml", "json")
    } else {
        false
    }
}

pub async fn handle_file(
    client: ApiClient<'_>,
    file: &PathBuf,
    noenv: bool,
) -> Result<(), Box<dyn Error>> {
    let mut fig = Figment::new();
    if !noenv {
        fig = fig.merge(Env::prefixed("TWITCHCTL_DEFAULT_"));
    }
    fig = match file.extension() {
        Some(ext) if matches_any!(ext, "yaml", "yml", "json") => fig.merge(Yaml::file(&file)),
        Some(ext) if matches_any!(ext, "ini", "toml") => fig.merge(Toml::file(&file)),
        Some(ext) => exit!(1, "Format not supported: `{}`.", ext.to_string_lossy()),
        None => exit!(1, "Config file needs an extension defining the format."),
    };
    if !noenv {
        fig = fig.merge(Env::prefixed("TWITCHCTL_"));
    }
    let config: Config = fig
        .extract()
        .unwrap_or_else(|e| exit!(1, "Failed to parse configuration: {:?}", e));
    // To not move config struct
    let tags = config.tags;
    let locale = config.config_locale;

    let tag_rq = async {
        if let Some(tags) = tags {
            client
                .replace_stream_tags(
                    client.get_user_id(),
                    client
                        .get_tag_ids_matching(
                            tags.as_slice(),
                            match locale.as_ref() {
                                Some(locale) => locale,
                                None => "en-us",
                            },
                        )
                        .await
                        .unwrap_or_else(|e| exit!(1, "Failed to request tags: {:?}", e)),
                )
                .await
                .unwrap_or_else(|e| exit!(1, "Failed to set tags: {:?}", e));
        }
    };
    if config.language.is_some() || config.title.is_some() || config.category.is_some() {
        let mut builder = ChannelInfoBuilder::default();
        if let Some(lang) = config.language {
            builder.language(lang);
        }
        if let Some(title) = config.title {
            builder.title(title);
        }
        if let Some(category) = config.category {
            builder.category(
                client
                    .search_category(&category)
                    .await
                    .unwrap_or_else(|e| exit!(1, "Failed to request category: {:?}", e))
                    .unwrap_or_else(|| exit!(1, "Could not find a category for `{}`", category))
                    .id,
            );
        }
        client
            .modify_channel_information(client.get_user_id(), builder.build().unwrap())
            .await
            .unwrap_or_else(|e| exit!(1, "Failed to set channel information: {:?}", e))
    }
    if let Some(_notification) = config.notification {
        warning!("Setting notification is not yet supported");
    }
    if let Some(rewards) = config.rewards {
        client
            .get_rewards(client.get_user_id())
            .await?
            .iter()
            .map(|CustomReward { id, .. }| {
                client.update_custom_reward(
                    client.get_user_id(),
                    id,
                    UpdateCustomRewardBody::builder().is_enabled(false).build(),
                )
            })
            .collect::<JoinAll<_>>()
            .await;

        rewards
            .iter()
            .map(|title| client.find_reward(client.get_user_id(), title))
            .collect::<JoinAll<_>>()
            .await
            .iter()
            .filter_map(|r| {
                if let Ok(Some(CustomReward { id, .. })) = r {
                    Some(client.update_custom_reward(
                        client.get_user_id(),
                        id,
                        UpdateCustomRewardBody::builder().is_enabled(true).build(),
                    ))
                } else {
                    None
                }
            })
            .collect::<JoinAll<_>>()
            .await;
    }
    tag_rq.await;

    Ok(())
}
