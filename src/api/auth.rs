use crate::core;
use crate::db;
use crate::{
    core::oauth::{self, OauthProvider},
    db::user::User,
};
use aws_sdk_dynamodb::Client;
use rocket::http::CookieJar;
use rocket::{serde::json::Json, State};
use uuid::Uuid;

#[get("/register?<auth_code>&<provider>")]
pub async fn register(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: handle corner cases
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    let user_info = oauth::get_info(&access_token, &provider).await;
    let user_id = Uuid::new_v4();
    let created_user = db::user::create(client, user_id, user_info, provider, uid).await;
    core::auth::set_logged_in_user_id(cookies, user_id);
    Json(created_user)
}

#[get("/login?<auth_code>&<provider>")]
pub async fn log_in(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: handle corner cases
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    let user_id = db::user::get_id(client, provider, uid).await.unwrap();
    core::auth::set_logged_in_user_id(cookies, user_id);
    Json(db::user::fetch(client, user_id).await)
}

#[get("/logout")]
pub async fn log_out(cookies: &CookieJar<'_>) -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: handle corner cases
    core::auth::unset_logged_in_user_id(cookies);
    Json(db::user::get_logged_out_user())
}

#[get("/connect?<auth_code>&<provider>")]
pub async fn connect_account(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    let user_id = core::auth::get_logged_in_user_id(cookies).unwrap();
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    Json(db::user::add_connected_account(client, user_id, provider, uid).await)
}

#[get("/me")]
pub async fn me(client: &State<Client>, cookies: &CookieJar<'_>) -> Json<User> {
    // TODO: handle unauthorised flows
    let user_id = core::auth::get_logged_in_user_id(cookies).unwrap();
    Json(db::user::fetch(client, user_id).await)
}
