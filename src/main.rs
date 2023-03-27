pub mod api;
pub mod core;
pub mod db;
pub mod model;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

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
            "/api/food_log",
            routes![
                api::food_logging::create,
                api::food_logging::fetch,
                api::food_logging::fetch_all,
                api::food_logging::edit,
                api::food_logging::delete
            ],
        )
        .mount(
            "/api/catalog",
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
            "/api/auth",
            routes![
                api::auth::log_in,
                api::auth::log_out,
                api::auth::register,
                api::auth::connect_account,
                api::auth::me,
            ],
        )
        .register(
            "/api/",
            catchers![
                api::fatal,
                api::not_authenticated,
                api::not_authorized,
                api::not_found,
                api::bad_request
            ],
        )
}
