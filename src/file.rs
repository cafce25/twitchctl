use crate::api::ChannelInfoBuilder;
use crate::ApiClient;
use crate::{exit, matches_any};
use figment::{
    providers::{Env, Format, Toml, Yaml},
    Figment,
};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Config {
    config_locale: Option<String>,
    tags: Option<Vec<String>>,
    language: Option<String>,
    title: Option<String>,
    category: Option<String>,
    notification: Option<String>,
}

pub async fn handle_file(
    client: ApiClient<'_>,
    file: PathBuf,
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
    let config: Config = fig.extract()?;

    if let Some(tags) = config.tags {
        client
            .replace_stream_tags(
                client.get_user_id(),
                client
                    .get_tag_ids_matching(
                        tags.as_slice(),
                        match config.config_locale.as_ref() {
                            Some(locale) => locale,
                            None => "en_us",
                        },
                    )
                    .await?,
            )
            .await?;
    }
    if config.language.is_some() || config.title.is_some() || config.category.is_some() {
        let mut builder = ChannelInfoBuilder::default();
        if let Some(lang) = config.language {
            builder.language(lang);
        }
        if let Some(title) = config.title {
            builder.title(title);
        }
        if let Some(category) = config.category {
            builder.game(
                client
                    .search_category(&category)
                    .await?
                    .unwrap_or_else(|| exit!(1, "Could not find a game for `{}`", category))
                    .id,
            );
        }
        client
            .modify_channel_information(client.get_user_id(), builder.build().unwrap())
            .await?;
    }
    if let Some(_notification) = config.notification {
        eprintln!("Setting notification is not yet supported");
    }

    Ok(())
}
