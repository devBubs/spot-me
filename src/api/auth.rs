use crate::config::RustyConfig;
use crate::core::response::respond;
use crate::core::{auth, oauth};
use crate::db;
use crate::model::io::ApiResponse;
use crate::model::{OauthProvider, User};
use aws_sdk_dynamodb::Client;
use rocket::http::CookieJar;
use rocket::{Route, State};
use uuid::Uuid;

pub fn get_all_routes() -> Vec<Route> {
    routes![register, log_in, log_out, me]
}

// TODO: merge login, register and connect into a single api

#[get("/<provider>/register?<code>")]
pub async fn register(
    provider: OauthProvider,
    code: String,
    client: &State<Client>,
    config: &State<RustyConfig>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<User> {
    // TODO: handle bad auth_code case
    // TODO: handle already registered case
    let access_token =
        oauth::get_access_token(code, &provider, config, oauth::SignUpType::REGISTER).await;
    let user_info = oauth::get_info(&access_token, &provider).await;
    let uid = &user_info.uid;
    let user_id = Uuid::new_v4();
    let created_user = db::user::create(client, user_id, &user_info, provider, uid).await;
    auth::set_logged_in_user_id(cookies, user_id)?;
    respond(created_user)
}

#[get("/<provider>/login?<code>")]
pub async fn log_in(
    provider: OauthProvider,
    code: String,
    client: &State<Client>,
    config: &State<RustyConfig>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<User> {
    // TODO: handle bad auth_code case
    // TODO: handle unregistered case
    let access_token =
        oauth::get_access_token(code, &provider, &config, oauth::SignUpType::LOGIN).await;
    let uid = oauth::get_info(&access_token, &provider).await.uid;
    let user_id = db::user::get_id(client, provider, uid).await?;
    auth::set_logged_in_user_id(cookies, user_id)?;
    respond(db::user::fetch(client, user_id).await)
}

#[get("/logout")]
pub async fn log_out(cookies: &CookieJar<'_>) -> ApiResponse<User> {
    auth::unset_logged_in_user_id(cookies)?;
    respond(db::user::get_logged_out_user())
}

#[get("/<provider>/connect?<auth_code>")]
pub async fn connect_account(
    provider: OauthProvider,
    auth_code: String,
    client: &State<Client>,
    config: &State<RustyConfig>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<User> {
    // TODO: handle bad auth_code case
    let user_id = auth::get_logged_in_user_id(cookies)?;
    let access_token =
        oauth::get_access_token(auth_code, &provider, &config, oauth::SignUpType::CONNECT).await;
    let uid = oauth::get_info(&access_token, &provider).await.uid;
    respond(db::user::add_connected_account(client, user_id, provider, uid).await)
}

#[get("/me")]
pub async fn me(client: &State<Client>, cookies: &CookieJar<'_>) -> ApiResponse<User> {
    if let Ok(user_id) = auth::get_logged_in_user_id(cookies) {
        respond(db::user::fetch(client, user_id).await)
    } else {
        respond(db::user::get_logged_out_user())
    }
}
