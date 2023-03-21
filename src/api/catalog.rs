use crate::core::auth;
use crate::core::response::{respond, ApiResponse};
use crate::db::{
    self,
    catalog::{CatalogItem, CatalogItemType, CatalogItemUpsertRequest},
};
use aws_sdk_dynamodb::Client;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

// TODO: Add support for User specific catalog items
// TODO: Add support for nested catalog items

#[post("/", data = "<input>")]
pub async fn create(
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<CatalogItem> {
    assert!(matches!(input.item_type, CatalogItemType::GLOBAL));
    let user_id = auth::get_logged_in_user_id(cookies)?;
    auth::assert_is_admin(user_id)?;

    let input = input.into_inner();
    respond(db::catalog::upsert(&client, Uuid::new_v4(), input).await)
}

#[get("/<id>")]
pub async fn fetch(
    id: Uuid,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<CatalogItem> {
    auth::get_logged_in_user_id(cookies)?;
    let item = db::catalog::fetch(&client, id).await;
    if let CatalogItemType::GLOBAL = item.item_type {
        respond(item)
    } else {
        Err(crate::core::response::ApiErrorType::AuthorizationError)
    }
}

#[get("/")]
pub async fn fetch_all(
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<Vec<CatalogItem>> {
    auth::get_logged_in_user_id(cookies)?;
    let items = db::catalog::fetch_all(&client).await;
    respond(filter_only_globals(items))
}

#[post("/<id>", data = "<input>")]
pub async fn edit(
    id: Uuid,
    input: Json<CatalogItemUpsertRequest>,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<CatalogItem> {
    let user_id = auth::get_logged_in_user_id(cookies)?;
    auth::assert_is_admin(user_id)?;
    respond(db::catalog::upsert(&client, id, input.into_inner()).await)
}

#[delete("/<id>")]
pub async fn delete(
    id: Uuid,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<bool> {
    let user_id = auth::get_logged_in_user_id(cookies)?;
    auth::assert_is_admin(user_id)?;
    respond(db::catalog::delete(&client, id).await)
}

#[get("/search?<prefix>")]
pub async fn search(
    prefix: Option<&str>,
    client: &State<Client>,
    cookies: &CookieJar<'_>,
) -> ApiResponse<Vec<CatalogItem>> {
    auth::get_logged_in_user_id(cookies)?;
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
