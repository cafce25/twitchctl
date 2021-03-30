mod api;
mod config;

use api::ApiClient;
use config::load_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let env = load_env();

    let client = ApiClient::new(env.token).await?;

    println!(
        "{:?}",
        client
            .search_categories("Minecraft".to_string(), None)
            .await?
    );

    Ok(())
}
