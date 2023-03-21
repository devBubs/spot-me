use crate::db;
use crate::{
    core::oauth::{self, OauthProvider},
    db::user::User,
};
use aws_sdk_dynamodb::Client;
use rocket::{serde::json::Json, State};
use uuid::Uuid;

#[get("/register?<auth_code>&<provider>")]
pub async fn register(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    let user_info = oauth::get_info(&access_token, &provider).await;
    let user_id = Uuid::new_v4();
    // TODO: set private cookie
    Json(db::user::create(client, user_id, user_info, provider, uid).await)
}

#[get("/login?<auth_code>&<provider>")]
pub async fn log_in(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    let user_id = db::user::get_id(client, provider, uid).await.unwrap();
    // TODO: set private cookie
    Json(db::user::fetch(client, user_id).await)
}

#[get("/logout")]
pub async fn log_out() -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: unset private cookie
    Json(db::user::get_logged_out_user())
}

#[get("/connect?<auth_code>&<provider>")]
pub async fn connect_account(
    auth_code: String,
    provider: OauthProvider,
    client: &State<Client>,
) -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: get user_id from private cookie
    let user_id = Uuid::new_v4();
    let access_token = oauth::get_access_token(auth_code, &provider).await;
    let uid = oauth::get_uid(&access_token, &provider).await;
    Json(db::user::add_connected_account(client, user_id, provider, uid).await)
}

#[get("/me")]
pub async fn me(client: &State<Client>) -> Json<User> {
    // TODO: handle unauthorised flows
    // TODO: get user_id from private cookie
    let user_id = Uuid::new_v4();
    Json(db::user::fetch(client, user_id).await)
}
