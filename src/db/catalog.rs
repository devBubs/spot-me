use std::collections::HashMap;

use aws_sdk_dynamodb::{
    model::{
        AttributeValue::{self, N, S},
        ReturnValue,
    },
    Client,
};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

const TABLE_NAME: &str = "spot-me.catalog";

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CatalogItem {
    pub id: Uuid,
    pub name: String,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub calories: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CatalogItemRequest {
    name: String,
    protein: f32,
    fat: f32,
    carbs: f32,
}

fn convert(output: &HashMap<String, AttributeValue>) -> CatalogItem {
    let id = Uuid::parse_str(output["id"].as_s().unwrap()).unwrap();
    let name = output["name"].as_s().unwrap().to_owned();
    let protein = output["protein"].as_n().unwrap().parse::<f32>().unwrap();
    let fat = output["fat"].as_n().unwrap().parse::<f32>().unwrap();
    let carbs = output["carbs"].as_n().unwrap().parse::<f32>().unwrap();
    let calories = output["carbs"].as_n().unwrap().parse::<i32>().unwrap();
    CatalogItem {
        id,
        name,
        protein,
        fat,
        carbs,
        calories,
    }
}

pub async fn fetch(client: &Client, id: Uuid) -> CatalogItem {
    let results = client
        .get_item()
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .send()
        .await
        .unwrap()
        .item()
        .unwrap()
        .clone();
    convert(&results)
}

pub async fn fetch_all(client: &Client) -> Vec<CatalogItem> {
    let output = client.scan().table_name(TABLE_NAME).send().await.unwrap();
    let items = output.items().unwrap();
    items.to_vec().iter().map(|v| convert(v)).collect()
}

pub async fn upsert(client: &Client, id: Uuid, input: CatalogItemRequest) -> CatalogItem {
    let calories = input.protein * 4.0 + input.carbs * 4.0 + input.fat * 8.0;
    let results = client
        .update_item()
        .return_values(ReturnValue::AllNew)
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression(
            "SET #name=:name, protein=:protein, fat=:fat, carbs=:carbs, calories=:calories",
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":name", S(input.name))
        .expression_attribute_values(":protein", N(input.protein.to_string()))
        .expression_attribute_values(":fat", N(input.fat.to_string()))
        .expression_attribute_values(":carbs", N(input.carbs.to_string()))
        .expression_attribute_values(":calories", N(calories.to_string()))
        .send()
        .await
        .unwrap()
        .attributes()
        .unwrap()
        .clone();
    convert(&results)
}

pub async fn delete(client: &Client, id: Uuid) -> bool {
    match client
        .delete_item()
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .send()
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
