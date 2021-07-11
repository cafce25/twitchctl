use structopt::StructOpt;

mod api;
mod cli;
mod config;
mod file;
mod preset;
mod rewards;
mod tags;

#[macro_use]
mod macros;

use api::ApiClient;
use cli::{Category, CliOptions};
use config::load_env;
use file::handle_file;
use preset::handle_preset;
use tags::tags;
use rewards::rewards;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CliOptions { category } = CliOptions::from_args();

    if let Category::Completions { shell, target_dir } = &category {
        if !target_dir.exists() {
            std::fs::create_dir_all(target_dir)?;
        }
        CliOptions::clap().gen_completions(env!("CARGO_PKG_NAME"), shell.into(), target_dir);
        println!(
            "Completions file has been written to the directory: {}",
            target_dir.to_string_lossy()
        );
        return Ok(());
    }

    // check token after cli and completions are done
    // otherwise the tool crashes when you try to call it with -h
    let env = load_env();
    let client = ApiClient::new(&env.token).await?;

    match category {
        Category::Tags { options } => tags(client, &options.locale, options.subcommand).await,
        Category::Search {
            category,
            max_results,
        } => {
            println!(
                "{:?}",
                client.search_categories(&category, max_results).await?
            );
        }
        Category::File { file, noenv } => handle_file(client, &file, noenv).await?,
        Category::Preset { query, noenv } => handle_preset(client, &query, noenv).await?,
        Category::Completions { .. } => {
            unreachable!("already handled above!")
        }
        Category::Reward { options } => rewards(client, options.subcommand).await,
    }

    Ok(())
}
