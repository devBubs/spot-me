use std::env;

use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize, Serialize, Debug)]
pub struct RustyConfig {
    pub google_redirection_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub aws_client_id: String,
    pub aws_client_secret: String,
    pub rocket_secret: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawRustyConfig {
    pub google_redirection_url: String,
    pub google_client_id_uri: String,
    pub google_client_secret_uri: String,
    pub aws_client_id_uri: String,
    pub aws_client_secret_uri: String,
    pub rocket_secret_key_uri: String,
}

pub async fn get_rusty_config() -> RustyConfig {
    let profile = if let true = cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let config: RawRustyConfig = Figment::from(Toml::file("./Rusty.toml"))
        .focus(profile)
        .extract()
        .expect("Config not found");
    let rocket_secret = fetch_from_file(&config.rocket_secret_key_uri).await;
    if let "release" = profile {
        env::set_var("ROCKET_SECRET_KEY", rocket_secret.clone());
    }
    RustyConfig {
        google_redirection_url: config.google_redirection_url,
        google_client_id: fetch_from_file(&config.google_client_id_uri).await,
        google_client_secret: fetch_from_file(&config.google_client_secret_uri).await,
        aws_client_id: fetch_from_file(&config.aws_client_id_uri).await,
        aws_client_secret: fetch_from_file(&config.aws_client_secret_uri).await,
        rocket_secret,
    }
}

async fn fetch_from_file(uri: &str) -> String {
    let mut file = File::open(uri).await.expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Failed to read file");
    contents.trim_end().to_owned()
}
