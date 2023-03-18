use aws_sdk_dynamodb::Client;
use rocket::State;

use crate::db::{
    self,
    catalog::{CatalogItem, CreateCatalogItemRequest},
};
use rocket::serde::json::Json;

#[post("/", data = "<input>")]
pub fn create(input: Json<CreateCatalogItemRequest>, client: &State<Client>) -> Json<CatalogItem> {
    Json(db::catalog::create(client, input.into_inner()))
    // "Catalog item logged"
}

#[get("/<id>")]
pub fn fetch(id: &str, client: &State<Client>) -> Json<CatalogItem> {
    Json(db::catalog::fetch(client, id))
    // format!("Catalog item fetched: {}", id)
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
