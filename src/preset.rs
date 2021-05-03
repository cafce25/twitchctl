use dirs;
use std::{error::Error, fs, path::PathBuf};

use crate::{
    api::ApiClient,
    exit,
    file::{handle_file, valid_extension},
};

pub async fn handle_preset(
    client: ApiClient<'_>,
    query: &str,
    noenv: bool,
) -> Result<(), Box<dyn Error>> {
    let mut config_dir = dirs::config_dir().unwrap_or_else(|| {
        exit!(
            1,
            "Could not find the config Home. Maybe set XDG_CONFIG_HOME"
        )
    });
    config_dir.push("twitchctl/presets");
    if !config_dir.is_dir() {
        if fs::create_dir_all(&config_dir).is_err() {
            exit!(
                1,
                "Unable to create preset directory at `{}`",
                config_dir.display()
            );
        }
    }

    let files: Vec<PathBuf> = fs::read_dir(&config_dir)
        .unwrap_or_else(|_| exit!(1, "Unable to read preset directory at `{}`", config_dir.display()))
        .filter_map(|res| {
            res.map_or(None, |file| {
                if valid_extension(&file.path())
                    && fuzzy_filter::matches(&query, &file.file_name().to_string_lossy())
                {
                    Some(file.path())
                } else {
                    None
                }
            })
        })
        .collect();
    if files.is_empty() {
        exit!(1, "No matching presets found.")
    }
    if files.len() > 1 {
        if let Some(strictly_filtered) = files.iter().find(|e| {
            e.file_name()
                .expect("all files have filenames")
                .to_string_lossy()
                == query
        }) {
            handle_file(client, strictly_filtered, noenv).await
        } else {
            exit!(
                1,
                "There where multiple files matching the query:\n{}",
                files
                    .iter()
                    .map(|f| f
                        .file_name()
                        .expect("All files have filenames")
                        .to_string_lossy()
                        .to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    } else {
        handle_file(
            client,
            files.first().expect("files should not be empty"),
            noenv,
        )
        .await
    }
}
