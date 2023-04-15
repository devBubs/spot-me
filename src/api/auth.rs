use crate::config::RustyConfig;
use crate::core::auth::{self, Authenticated};
use crate::core::oauth;
use crate::core::response::respond;
use crate::db;
use crate::model::io::ApiResponse;
use crate::model::{AccessToken, OauthProvider, User};
use aws_sdk_dynamodb::Client;
use rocket::{Route, State};

pub fn get_all_routes() -> Vec<Route> {
    routes![connect, me]
}

// TODO: implement shortlived access_token and refresh tokens

#[get("/connect?<token>&<provider>")]
pub async fn connect(
    token: String,
    provider: OauthProvider,
    client: &State<Client>,
    config: &State<RustyConfig>,
) -> ApiResponse<AccessToken> {
    let user_info = oauth::get_info(&token, &provider).await;
    let id = db::user::get_or_create(client, &provider, &user_info).await;
    respond(auth::get_access_token(&id, &config.inner().rocket_secret))
}

#[get("/me")]
pub async fn me(client: &State<Client>, credentials: Authenticated) -> ApiResponse<Option<User>> {
    let user = db::user::fetch(client, credentials.user_id).await;
    respond(Some(user))
}

// TODO: figure out how to connect other OAuth accounts
// #[get("/<provider>/connect?<auth_code>")]
// pub async fn connect_account(
//     provider: OauthProvider,
//     auth_code: String,
//     client: &State<Client>,
//     config: &State<RustyConfig>,
//     cookies: &CookieJar<'_>,
// ) -> ApiResponse<User> {
//     // TODO: handle bad auth_code case
//     let user_id = auth::get_logged_in_user_id(cookies)?;
//     let access_token =
//         oauth::get_access_token(auth_code, &provider, &config, oauth::SignUpType::CONNECT).await;
//     let uid = oauth::get_info(&access_token, &provider).await.uid;
//     respond(db::user::add_connected_account(client, user_id, provider, uid).await)
// }
