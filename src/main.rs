mod api;
mod config;

use api::get_user;
use config::{load_env, DotEnv};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let env = load_env();

    let user = get_user(env.token).await?;

    println!("{}", user.id);

    Ok(())
}
