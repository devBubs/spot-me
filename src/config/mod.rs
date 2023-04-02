use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize, Serialize, Debug)]
pub struct RustyConfig {
    // TODO: merge login, register and connect into a single api
    pub google_redirection_url_login: String,
    pub google_redirection_url_register: String,
    pub google_redirection_url_connect: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub aws_client_id: String,
    pub aws_client_secret: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawRustyConfig {
    // TODO: merge login, register and connect into a single api
    pub google_redirection_url_login: String,
    pub google_redirection_url_register: String,
    pub google_redirection_url_connect: String,
    pub google_client_id_uri: String,
    pub google_client_secret_uri: String,
    pub aws_client_id_uri: String,
    pub aws_client_secret_uri: String,
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
    RustyConfig {
        google_redirection_url_login: config.google_redirection_url_login,
        google_redirection_url_register: config.google_redirection_url_register,
        google_redirection_url_connect: config.google_redirection_url_connect,
        google_client_id: fetch_from_file(&config.google_client_id_uri).await,
        google_client_secret: fetch_from_file(&config.google_client_secret_uri).await,
        aws_client_id: fetch_from_file(&config.aws_client_id_uri).await,
        aws_client_secret: fetch_from_file(&config.aws_client_secret_uri).await,
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
