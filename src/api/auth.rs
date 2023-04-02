use crate::config::RustyConfig;
use crate::core::response::respond;
use crate::core::{auth, oauth};
use crate::db;
use crate::model::io::ApiResponse;
use crate::model::{OauthProvider, User};
use aws_sdk_dynamodb::Client;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::{Route, State};

pub fn get_all_routes() -> Vec<Route> {
    routes![connect, log_out, me]
}

#[get("/<provider>/connect?<code>")]
pub async fn connect(
    provider: OauthProvider,
    code: String,
    client: &State<Client>,
    config: &State<RustyConfig>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    // TODO: handle bad auth_code case
    // TODO: handle already registered case
    let access_token = oauth::get_access_token(code, &provider, config).await;
    let user_info = oauth::get_info(&access_token, &provider).await;
    let user = db::user::get_or_create(client, &provider, &user_info).await;
    auth::set_logged_in_user_id(cookies, user.id).unwrap();
    Redirect::to(uri!("/"))
}

#[get("/logout")]
pub async fn log_out(cookies: &CookieJar<'_>) -> ApiResponse<User> {
    auth::unset_logged_in_user_id(cookies)?;
    respond(db::user::get_logged_out_user())
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

#[get("/me")]
pub async fn me(client: &State<Client>, cookies: &CookieJar<'_>) -> ApiResponse<User> {
    if let Ok(user_id) = auth::get_logged_in_user_id(cookies) {
        respond(db::user::fetch(client, user_id).await)
    } else {
        respond(db::user::get_logged_out_user())
    }
}
