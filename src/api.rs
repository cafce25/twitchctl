use surf::Client;
use twitch_api2::HelixClient;
use twitch_oauth2::{client::surf_http_client, AccessToken, TwitchToken, UserToken};

pub struct User {
    pub id: String,
    pub token: UserToken,
}

pub async fn get_user(token_string: String) -> Result<User, Box<dyn std::error::Error + 'static>> {
    let token =
        UserToken::from_existing(surf_http_client, AccessToken::new(token_string), None, None)
            .await?;

    let id = token
        .validate_token(surf_http_client)
        .await?
        .user_id
        .unwrap();

    Ok(User { id, token })
}
