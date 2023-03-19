use aws_sdk_dynamodb::Client;
use rocket::State;
use uuid::Uuid;

use crate::db::{
    self,
    catalog::{CatalogItem, CreateCatalogItemRequest},
};
use rocket::serde::json::Json;

#[post("/", data = "<input>")]
pub async fn create(
    input: Json<CreateCatalogItemRequest>,
    client: &State<Client>,
) -> Json<CatalogItem> {
    Json(db::catalog::create(&client, input.into_inner()).await)
}

#[get("/<id>")]
pub async fn fetch(id: Uuid, client: &State<Client>) -> Json<CatalogItem> {
    Json(db::catalog::fetch(&client, id).await)
}

#[get("/")]
pub async fn fetch_all(client: &State<Client>) -> Json<Vec<CatalogItem>> {
    Json(db::catalog::fetch_all(&client).await)
}

#[post("/<id>")]
pub fn edit(id: &str) -> String {
    format!("Catalog item edited: {}", id)
}

#[delete("/<id>")]
pub fn delete(id: &str) -> String {
    format!("Catalog item deleted: {}", id)
}
