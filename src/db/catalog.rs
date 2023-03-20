use aws_sdk_dynamodb::{
    model::{
        AttributeValue::{self, N, S},
        ReturnValue,
    },
    Client,
};
use rocket::serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr};
use uuid::Uuid;

const TABLE_NAME: &str = "spot-me.catalog";

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum CatalogItemType {
    GLOBAL = 0,
    USER = 1,
}

impl std::str::FromStr for CatalogItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GLOBAL" => Ok(CatalogItemType::GLOBAL),
            "USER" => Ok(CatalogItemType::USER),
            _ => Err(format!("'{}' is not a valid value for CatalogItemType", s)),
        }
    }
}

impl fmt::Display for CatalogItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CatalogItemType::GLOBAL => write!(f, "GLOBAL"),
            CatalogItemType::USER => write!(f, "USER"),
        }
    }
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
pub struct CatalogItemRequest {
    pub name: String,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub item_type: CatalogItemType,
}

fn convert(output: &HashMap<String, AttributeValue>) -> CatalogItem {
    let id = Uuid::parse_str(output["id"].as_s().unwrap()).unwrap();
    let name = output["name"].as_s().unwrap().to_owned();
    let protein = output["protein"].as_n().unwrap().parse::<f32>().unwrap();
    let fat = output["fat"].as_n().unwrap().parse::<f32>().unwrap();
    let carbs = output["carbs"].as_n().unwrap().parse::<f32>().unwrap();
    let calories = output["carbs"].as_n().unwrap().parse::<i32>().unwrap();
    let item_type = CatalogItemType::from_str(output["item_type"].as_s().unwrap()).unwrap();
    let user_id = match item_type {
        CatalogItemType::GLOBAL => None,
        CatalogItemType::USER => Some(Uuid::parse_str(output["user_id"].as_s().unwrap()).unwrap()),
    };
    CatalogItem {
        id,
        name,
        protein,
        fat,
        carbs,
        calories,
        item_type,
        user_id,
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
            "SET #name=:name, protein=:protein, fat=:fat, carbs=:carbs, calories=:calories, item_type=:item_type",
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":name", S(input.name))
        .expression_attribute_values(":protein", N(input.protein.to_string()))
        .expression_attribute_values(":fat", N(input.fat.to_string()))
        .expression_attribute_values(":carbs", N(input.carbs.to_string()))
        .expression_attribute_values(":calories", N(calories.to_string()))
        .expression_attribute_values(":item_type", S(input.item_type.to_string()))
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

pub async fn search(client: &Client, prefix: &str) -> Vec<CatalogItem> {
    let output = client
        .scan()
        .table_name(TABLE_NAME)
        .filter_expression("begins_with (#name, :prefix)")
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":prefix", S(prefix.to_owned()))
        .send()
        .await
        .unwrap();
    let items = output.items().unwrap();
    items.to_vec().iter().map(|v| convert(v)).collect()
}
