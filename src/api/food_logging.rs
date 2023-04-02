use rocket::Route;

pub fn get_all_routes() -> Vec<Route> {
    routes![create, fetch, fetch_all, edit, delete]
}

#[post("/")]
pub fn create() -> &'static str {
    "Food logged"
}

#[get("/<id>")]
pub fn fetch(id: &str) -> String {
    format!("Food log fetched: {}", id)
}

#[get("/")]
pub fn fetch_all() -> &'static str {
    "Fetched all food logs"
}

#[post("/<id>")]
pub fn edit(id: &str) -> String {
    format!("Food log edited: {}", id)
}

#[delete("/<id>")]
pub fn delete(id: &str) -> String {
    format!("Food log deleted: {}", id)
}
