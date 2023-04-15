pub mod io;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct OauthUserInfo {
    pub uid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize, FromFormField)]
pub enum OauthProvider {
    GOOGLE = 0,
    GITHUB = 1,
    MICROSOFT = 2,
    FACEBOOK = 3,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub picture: String,
    pub connected_accounts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
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
pub enum CatalogItemType {
    GLOBAL = 0,
    USER = 1,
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}
