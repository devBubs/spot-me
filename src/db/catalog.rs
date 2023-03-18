use aws_sdk_dynamodb::Client;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CatalogItem {
    id: Uuid,
    name: String,
    protein: f32,
    fat: f32,
    carbs: f32,
    calories: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCatalogItemRequest {
    name: String,
    protein: f32,
    fat: f32,
    carbs: f32,
}

pub fn fetch(_client: &Client, _id: &str) -> CatalogItem {
    CatalogItem {
        id: Uuid::new_v4(),
        name: "Rice".to_owned(),
        protein: 10.0,
        fat: 0.1,
        carbs: 50.0,
        calories: 241,
    }
}

pub fn create(_client: &Client, input: CreateCatalogItemRequest) -> CatalogItem {
    let calories = input.protein * 4.0 + input.carbs * 4.0 + input.fat * 8.0;
    CatalogItem {
        id: Uuid::new_v4(),
        name: input.name,
        protein: input.protein,
        fat: input.fat,
        carbs: input.carbs,
        calories: calories as i32,
    }
}
