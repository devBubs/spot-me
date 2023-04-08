use crate::core::auth::{self, Authenticated};
use crate::core::response::respond;
use crate::db;
use crate::model::io::{ApiErrorType, ApiResponse, CatalogItemUpsertRequest};
use crate::model::{CatalogItem, CatalogItemType};
use aws_sdk_dynamodb::Client;
use rocket::serde::json::Json;
use rocket::{Route, State};
use uuid::Uuid;

// TODO: Add support for User specific catalog items
// TODO: Add support for nested catalog items

pub fn get_all_routes() -> Vec<Route> {
    routes![create, fetch, fetch_all, edit, delete, search]
}

#[post("/", data = "<input>")]
pub async fn create(
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
    credentials: Authenticated,
) -> ApiResponse<CatalogItem> {
    assert!(matches!(input.item_type, CatalogItemType::GLOBAL));
    let user_id = credentials.user_id;
    auth::assert_is_admin(user_id)?;
    respond(db::catalog::upsert(&client, Uuid::new_v4(), input.into_inner()).await)
}

#[get("/<id>")]
pub async fn fetch(
    id: Uuid,
    client: &State<Client>,
    _credentials: Authenticated,
) -> ApiResponse<CatalogItem> {
    let item = db::catalog::fetch(&client, id).await;
    if let CatalogItemType::GLOBAL = item.item_type {
        respond(item)
    } else {
        Err(ApiErrorType::AuthorizationError)
    }
}

#[get("/")]
pub async fn fetch_all(
    client: &State<Client>,
    _credentials: Authenticated,
) -> ApiResponse<Vec<CatalogItem>> {
    let items = db::catalog::fetch_all(&client).await;
    respond(filter_only_globals(items))
}

#[post("/<id>", data = "<input>")]
pub async fn edit(
    id: Uuid,
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
    credentials: Authenticated,
) -> ApiResponse<CatalogItem> {
    let user_id = credentials.user_id;
    auth::assert_is_admin(user_id)?;
    respond(db::catalog::upsert(&client, id, input.into_inner()).await)
}

#[delete("/<id>")]
pub async fn delete(
    id: Uuid,
    client: &State<Client>,
    credentials: Authenticated,
) -> ApiResponse<bool> {
    let user_id = credentials.user_id;
    auth::assert_is_admin(user_id)?;
    respond(db::catalog::delete(&client, id).await)
}

#[get("/search?<prefix>")]
pub async fn search(
    prefix: Option<&str>,
    client: &State<Client>,
    _credentials: Authenticated,
) -> ApiResponse<Vec<CatalogItem>> {
    let items = match prefix {
        Some(prefix) => db::catalog::search(&client, prefix).await,
        None => db::catalog::fetch_all(&client).await,
    };
    respond(filter_only_globals(items))
}

// TODO: remove once user specific catalog is implemented
fn filter_only_globals(items: Vec<CatalogItem>) -> Vec<CatalogItem> {
    items
        .into_iter()
        .filter(|i| {
            i.item_type
                .to_string()
                .eq(&CatalogItemType::GLOBAL.to_string())
        })
        .collect()
}
