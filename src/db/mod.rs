pub mod catalog;
pub mod food_logging;
pub mod user;

use aws_sdk_dynamodb::{Client, Region};
use std::env;

use crate::config::RustyConfig;

pub async fn get_ddb_client(config: &RustyConfig) -> Client {
    env::set_var("AWS_ACCESS_KEY_ID", config.aws_client_id.clone());
    env::set_var("AWS_SECRET_ACCESS_KEY", config.aws_client_secret.clone());
    let aws_config = aws_config::from_env()
        .region(Region::new("eu-west-2"))
        .load()
        .await;
    Client::new(&aws_config)
}
