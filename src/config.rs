use dotenv;

const TOKEN_VAR: &str = "TWITCHCTL_TOKEN";

pub struct DotEnv {
    pub token: String,
}

pub fn load_env() -> DotEnv {
    let token = match dotenv::var(TOKEN_VAR) {
        Ok(token) => token,
        Err(_) => panic!(
            "Your Twitch oauth token is missing!\r\n{} should exist in .env or your env vars.",
            TOKEN_VAR
        ),
    };

    DotEnv { token }
}
