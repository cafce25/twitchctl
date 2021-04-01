mod api;
mod config;
mod tags;

use api::ApiClient;
use clap::App;
use config::load_env;
use tags::{tags, get_tags_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = load_env();

    let client = ApiClient::new(env.token).await?;

    let matches = App::new("twitchctl")
        .version("0.1.0")
        .about("A sane Twitch commandline interface")
        .subcommand(
            get_tags_app()
        )
        .subcommand(App::new("search_categories").about("searches in categories"))
        .get_matches();

    match matches.subcommand() {
        ("tags", Some(sub_m)) => tags(client, sub_m).await,
        ("search_categories", _) => {
            println!(
                "{:?}",
                client
                    .search_categories("Minecraft".to_string(), None)
                    .await?
            );
        }
        _ => {}
    }

    Ok(())
}
