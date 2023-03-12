#[post("/")]
pub fn create() -> &'static str {
    "Catalog item logged"
}

#[get("/<id>")]
pub fn fetch(id: &str) -> String {
    format!("Catalog item fetched: {}", id)
}

#[get("/")]
pub fn fetch_all() -> &'static str {
    "Fetched all Catalog items"
}

#[post("/<id>")]
pub fn edit(id: &str) -> String {
    format!("Catalog item edited: {}", id)
}

#[delete("/<id>")]
pub fn delete(id: &str) -> String {
    format!("Catalog item deleted: {}", id)
}
