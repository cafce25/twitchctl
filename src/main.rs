mod config;

use config::{load_env, DotEnv};

fn main() {
    let fake_env_var = load_env();
    println!("Hello, {}!", fake_env_var.token);
}
