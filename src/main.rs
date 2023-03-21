pub mod api;
pub mod core;
pub mod db;

use crate::core::response::ApiError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    std::env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    rocket::build()
        .manage(client)
        .mount(
            "/food_log",
            routes![
                api::food_logging::create,
                api::food_logging::fetch,
                api::food_logging::fetch_all,
                api::food_logging::edit,
                api::food_logging::delete
            ],
        )
        .mount(
            "/catalog",
            routes![
                api::catalog::create,
                api::catalog::fetch,
                api::catalog::fetch_all,
                api::catalog::edit,
                api::catalog::delete,
                api::catalog::search,
            ],
        )
        .mount(
            "/auth",
            routes![
                api::auth::log_in,
                api::auth::log_out,
                api::auth::register,
                api::auth::connect_account,
                api::auth::me,
            ],
        )
        .register(
            "/",
            catchers![
                fatal,
                not_authenticated,
                not_authorized,
                not_found,
                bad_request
            ],
        )
}

#[catch(500)]
fn fatal() -> Json<ApiError> {
    Json(ApiError {
        status_code: 500,
        error: "Not your fault. We will look into it. Sorry!".to_owned(),
    })
}

#[catch(401)]
fn not_authenticated() -> Json<ApiError> {
    Json(ApiError {
        status_code: 401,
        error: "The request is not authenticated.".to_owned(),
    })
}

#[catch(403)]
fn not_authorized() -> Json<ApiError> {
    Json(ApiError {
        status_code: 403,
        error: "The request is not authorized to perform this action.".to_owned(),
    })
}

#[catch(404)]
fn not_found() -> Json<ApiError> {
    Json(ApiError {
        status_code: 404,
        error: "The resource you are looking for does not exist.".to_owned(),
    })
}

#[catch(400)]
fn bad_request() -> Json<ApiError> {
    Json(ApiError {
        status_code: 400,
        error: "The request parameters appears to be malformed.".to_owned(),
    })
}
