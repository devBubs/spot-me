use aws_sdk_dynamodb::Client;
use rocket::State;
use uuid::Uuid;

use crate::db::{
    self,
    catalog::{CatalogItem, CatalogItemType, CatalogItemUpsertRequest},
};
use rocket::serde::json::Json;

#[post("/", data = "<input>")]
pub async fn create(
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
) -> Json<CatalogItem> {
    let input = input.into_inner();
    assert!(matches!(input.item_type, CatalogItemType::GLOBAL));
    Json(db::catalog::upsert(&client, Uuid::new_v4(), input).await)
}

#[get("/<id>")]
pub async fn fetch(id: Uuid, client: &State<Client>) -> Json<CatalogItem> {
    Json(db::catalog::fetch(&client, id).await)
}

#[get("/")]
pub async fn fetch_all(client: &State<Client>) -> Json<Vec<CatalogItem>> {
    Json(db::catalog::fetch_all(&client).await)
}

#[post("/<id>", data = "<input>")]
pub async fn edit(
    id: Uuid,
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
) -> Json<CatalogItem> {
    Json(db::catalog::upsert(&client, id, input.into_inner()).await)
}

#[delete("/<id>")]
pub async fn delete(id: Uuid, client: &State<Client>) -> Json<bool> {
    Json(db::catalog::delete(&client, id).await)
}

#[get("/search?<prefix>")]
pub async fn search(prefix: Option<&str>, client: &State<Client>) -> Json<Vec<CatalogItem>> {
    match prefix {
        Some(prefix) => Json(db::catalog::search(&client, prefix).await),
        None => Json(db::catalog::fetch_all(&client).await),
    }
}
