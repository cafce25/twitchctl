use std::error::Error;
use twitch_api2::{
    helix::{
        search::{search_categories::Category, SearchCategoriesRequest},
        streams::{
            get_stream_tags::GetStreamTagsRequest,
            replace_stream_tags::{ReplaceStreamTags, ReplaceStreamTagsRequest, ReplaceStreamTagsBody},
        },
        tags::{GetAllStreamTagsRequest, TwitchTag},
        users::{GetUsersRequest, User},
    },
    twitch_oauth2::{AccessToken, TwitchToken, UserToken},
    types::{UserId, UserName},
    HelixClient,
};
use twitch_oauth2::client::surf_http_client;

use derivative::Derivative;

#[derive(thiserror::Error, Debug)]
enum ApiError {
    #[error("No user with login `{0}` found.")]
    NoUser(String),
}

pub enum UserIdent {
    UserName(UserName),
    UserId(UserId),
    None,
}

async fn get_user(token_string: &str) -> Result<UserToken, Box<dyn Error + 'static>> {
    let token = UserToken::from_existing(
        surf_http_client,
        AccessToken::new(token_string.to_string()),
        None,
        None,
    )
    .await?;
    token.validate_token(surf_http_client).await?;

    Ok(token)
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ApiClient<'a> {
    #[derivative(Debug = "ignore")]
    helix_client: HelixClient<'a, surf::Client>,
    token: UserToken,
}

impl<'a> ApiClient<'a> {
    pub async fn new(token: &str) -> Result<ApiClient<'a>, Box<dyn Error>> {
        Ok(ApiClient {
            helix_client: HelixClient::with_client(surf::Client::new()),
            token: get_user(token).await?,
        })
    }

    pub fn get_user(&self) -> &str {
        self.token.login.as_ref()
    }

    pub fn get_user_id(&self) -> &str {
        &self.token.user_id
    }

    pub async fn search_categories(
        &self,
        term: &str,
        max: usize,
    ) -> Result<Option<Vec<Category>>, Box<dyn Error>> {
        // TODO Implement some better filter (only starting with for example) to reduce the number
        // of results for searches

        // TODO Maybe only return Some(Category) for one result.

        let req = SearchCategoriesRequest::builder()
            .query(term)
            .first(max.max(1).min(100).to_string())
            .build();
        let res: Vec<Category> = self.helix_client.req_get(req, &self.token).await?.data;
        if res.len() > 0 {
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub async fn get_users(
        &self,
        user_names: &[&UserName],
        user_ids: &[&UserId],
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let user_names: Vec<UserName> = user_names.iter().map(|n| n.to_string()).collect();
        let user_ids: Vec<UserName> = user_ids.iter().map(|n| n.to_string()).collect();
        let req = match (user_names.len(), user_ids.len()) {
            (0, 0) => GetUsersRequest::builder().build(),
            (_, 0) => GetUsersRequest::builder().login(user_names).build(),
            (0, _) => GetUsersRequest::builder().id(user_ids).build(),
            _ => GetUsersRequest::builder()
                .id(user_ids.into())
                .login(user_names.into())
                .build(),
        };

        let res: Vec<User> = self.helix_client.req_get(req, &self.token).await?.data;
        Ok(res)
    }

    pub async fn replace_stream_tags(
        &self,
        broadcaster_id: String,
        tag_ids: Vec<String>,
    ) -> Result<ReplaceStreamTags, Box<dyn Error + 'static>> {
        let req = ReplaceStreamTagsRequest::builder().broadcaster_id(broadcaster_id).build();
        let body = ReplaceStreamTagsBody::builder().tag_ids(tag_ids).build();
        let res = self.helix_client.req_put(req, body, &self.token).await?;
        Ok(res)
    }

    pub async fn get_stream_tags(
        &self,
        id: String,
    ) -> Result<Vec<TwitchTag>, Box<dyn Error>> {
        let tag_req = GetStreamTagsRequest::builder().broadcaster_id(id).build();
        let tag_res = self.helix_client.req_get(tag_req, &self.token).await?;
        Ok(tag_res.data)
    }

    pub async fn get_all_tags(&self) -> Result<Vec<TwitchTag>, Box<dyn Error>> {
        let mut tags = vec![];
        let mut pagination = None;
        loop {
            let req = GetAllStreamTagsRequest::builder()
                .after(pagination)
                .first(Some(100))
                .build();
            let mut res = self.helix_client.req_get(req, &self.token).await?;
            tags.append(&mut res.data);
            pagination = res.pagination;
            if pagination == None {
                break;
            }
        }
        Ok(tags)
    }

    pub async fn get_broadcaster_id(
        &self,
        broadcaster_ident: UserIdent,
    ) -> Result<UserId, Box<dyn Error>> {
        match broadcaster_ident {
            UserIdent::None => Ok(self.get_user_id().to_string()),
            UserIdent::UserId(broadcaster_id) => Ok(broadcaster_id),
            UserIdent::UserName(broadcaster_name) => {
                match self.get_users(&[&broadcaster_name], &[]).await {
                    Ok(userlist) => {
                        if userlist.is_empty() {
                            Err(Box::new(ApiError::NoUser(broadcaster_name)))
                        } else {
                            Ok(userlist[0].id.clone())
                        }
                    }
                    Err(e) => { Err(e) }
                }
            }
        }
    }
}
