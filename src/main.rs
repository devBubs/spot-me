#[macro_use]
extern crate rocket;

pub mod api;
use crate::api::catalog;
use crate::api::food_logging;

#[launch]
fn rocket() -> _ {
    rocket::build()
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
            ],
        )
}
