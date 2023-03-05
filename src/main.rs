#[macro_use]
extern crate rocket;

#[post("/food_log")]
fn create_entry() -> &'static str {
    "Food logged"
}

#[get("/food_log/<id>")]
fn fetch_entry(id: &str) -> &'static str {
    "Food log fetched- ..."
}

#[get("/food_log")]
fn fetch_all_entries() -> &'static str {
    "Fetched all food logs"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_entry, fetch_entry, fetch_all_entries])
}
