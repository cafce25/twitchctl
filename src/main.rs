use structopt::StructOpt;

mod api;
mod cli;
mod config;
mod tags;

use api::ApiClient;
use cli::{CliOptions, SubCommand};
use config::load_env;
use tags::tags;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CliOptions { subcommand } = CliOptions::from_args();

    if let SubCommand::Completions { shell, target_dir } = &subcommand {
        if !target_dir.exists() {
            std::fs::create_dir_all(target_dir)?;
        }
        CliOptions::clap().gen_completions(env!("CARGO_PKG_NAME"), shell.into(), target_dir);
        println!("Completions file has been written to the directory: {}", target_dir.to_string_lossy());
        return Ok(());
    }

    let env = load_env();

    let client = ApiClient::new(env.token).await?;

    match subcommand {
        SubCommand::Tags { options } => tags(client, &options.locale, options.subcommand).await,
        SubCommand::SearchCategories => {
            println!(
                "{:?}",
                client
                    .search_categories("Minecraft".to_string(), None)
                    .await?
            );
        }
        SubCommand::Completions { .. } => {
            unreachable!("already handled above!")
        }
    }

    Ok(())
}
