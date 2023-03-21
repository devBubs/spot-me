use crate::core::response::{respond, ApiResponse};
use crate::db::{
    self,
    catalog::{CatalogItem, CatalogItemType, CatalogItemUpsertRequest},
};
use aws_sdk_dynamodb::Client;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[post("/", data = "<input>")]
pub async fn create(
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
) -> ApiResponse<CatalogItem> {
    let input = input.into_inner();
    assert!(matches!(input.item_type, CatalogItemType::GLOBAL));
    respond(db::catalog::upsert(&client, Uuid::new_v4(), input).await)
}

#[get("/<id>")]
pub async fn fetch(id: Uuid, client: &State<Client>) -> ApiResponse<CatalogItem> {
    respond(db::catalog::fetch(&client, id).await)
}

#[get("/")]
pub async fn fetch_all(client: &State<Client>) -> ApiResponse<Vec<CatalogItem>> {
    respond(db::catalog::fetch_all(&client).await)
}

#[post("/<id>", data = "<input>")]
pub async fn edit(
    id: Uuid,
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
) -> ApiResponse<CatalogItem> {
    respond(db::catalog::upsert(&client, id, input.into_inner()).await)
}

#[delete("/<id>")]
pub async fn delete(id: Uuid, client: &State<Client>) -> ApiResponse<bool> {
    respond(db::catalog::delete(&client, id).await)
}

#[get("/search?<prefix>")]
pub async fn search(prefix: Option<&str>, client: &State<Client>) -> ApiResponse<Vec<CatalogItem>> {
    match prefix {
        Some(prefix) => respond(db::catalog::search(&client, prefix).await),
        None => respond(db::catalog::fetch_all(&client).await),
    }
}
