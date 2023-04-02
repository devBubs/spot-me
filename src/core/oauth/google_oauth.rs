use std::collections::HashMap;

use crate::model::OauthUserInfo;
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

pub async fn get_access_token(
    auth_code: String,
    redirect_uri: &str,
    client_id: &str,
    client_secret: &str,
) -> String {
    let mut params = HashMap::new();
    params.insert("code", auth_code);
    params.insert("client_id", client_id.to_owned());
    params.insert("client_secret", client_secret.to_owned());
    params.insert("redirect_uri", redirect_uri.to_owned());
    params.insert("grant_type", "authorization_code".to_owned());
    let client = reqwest::Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    let token_response: serde_json::Value = serde_json::from_str(&body).unwrap();
    let access_token = token_response["access_token"].as_str().unwrap();
    access_token.to_owned()
}

#[derive(Debug, Deserialize, Serialize)]
struct Name {
    #[serde(alias = "givenName")]
    first_name: Option<String>,
    #[serde(alias = "familyName")]
    last_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Email {
    metadata: EmailMetadata,
    value: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct EmailMetadata {
    source: EmailMetadataSource,
}

#[derive(Debug, Deserialize, Serialize)]
struct EmailMetadataSource {
    id: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    names: Option<Vec<Name>>,
    #[serde(alias = "emailAddresses")]
    email_addresses: Option<Vec<Email>>,
}
pub async fn get_info(access_token: &str) -> OauthUserInfo {
    let client = reqwest::Client::new();
    let res = client
        .get("https://people.googleapis.com/v1/people/me?personFields=names,emailAddresses")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    let person: Person = serde_json::from_str(&body).unwrap();
    let name = person.names.as_ref().and_then(|n| n.first()).unwrap();
    let email = person
        .email_addresses
        .as_ref()
        .and_then(|e| e.first())
        .unwrap();
    OauthUserInfo {
        uid: email.metadata.source.id.clone(),
        first_name: name.first_name.as_ref().unwrap().clone(),
        last_name: name.last_name.as_ref().unwrap().clone(),
        email: email.value.as_ref().unwrap().clone(),
    }
}
