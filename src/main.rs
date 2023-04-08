pub mod api;
pub mod config;
pub mod core;
pub mod db;
pub mod model;

#[macro_use]
extern crate rocket;

// TODO: remove all unwraps
// TODO: convert all sync ios to async
// TODO: switch to newer module format
// TODO: refactor all use statements
// TODO: use references instead of moving data

#[launch]
async fn rocket() -> _ {
    let config = config::get_rusty_config().await;
    rocket::build()
        .manage(db::get_ddb_client(&config).await)
        .manage(config)
        .mount("/api/food_log", api::food_logging::get_all_routes())
        .mount("/api/catalog", api::catalog::get_all_routes())
        .mount("/api/auth", api::auth::get_all_routes())
        .mount("/", api::get_all_static_routes())
        .register("/api/", api::get_all_catchers())
        // TODO: verify if this is safe
        .attach(api::CorsFairing)
        .attach(api::NoCacheFairing)
}
