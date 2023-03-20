use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod db;

use crate::api::catalog;
use crate::api::food_logging;

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
                food_logging::create,
                food_logging::fetch,
                food_logging::fetch_all,
                food_logging::edit,
                food_logging::delete
            ],
        )
        .mount(
            "/catalog",
            routes![
                catalog::create,
                catalog::fetch,
                catalog::fetch_all,
                catalog::edit,
                catalog::delete,
                catalog::search,
            ],
        )
}
