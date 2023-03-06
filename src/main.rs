#[macro_use]
extern crate rocket;

#[post("/food_log")]
fn log_food() -> &'static str {
    "Food logged"
}

#[get("/food_log/<id>")]
fn fetch_food_log(id: &str) -> String {
    format!("Food log fetched: {}", id)
}

#[get("/food_log")]
fn fetch_all_food_logs() -> &'static str {
    "Fetched all food logs"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![log_food, fetch_food_log, fetch_all_food_logs])
}
