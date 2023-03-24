use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod io;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OauthUserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum OauthProvider {
    GOOGLE = 0,
    GITHUB = 1,
    MICROSOFT = 2,
    FACEBOOK = 3,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub connected_accounts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CatalogItem {
    pub id: Uuid,
    pub item_type: CatalogItemType,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub calories: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum CatalogItemType {
    GLOBAL = 0,
    USER = 1,
}
