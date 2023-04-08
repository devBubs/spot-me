use crate::{
    config::RustyConfig,
    model::{io::ApiErrorType, AccessToken},
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{
    request::{self, FromRequest},
    Request,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

const ADMIN_USER_IDS: &[&str] = &["r2's ID", "c2's ID"];

// TODO: best to design something less childish soon
pub fn assert_is_admin(id: Uuid) -> Result<(), ApiErrorType> {
    if let true = ADMIN_USER_IDS
        .iter()
        .any(|&admin_id| id.to_string().eq(admin_id))
    {
        Ok(())
    } else {
        Err(ApiErrorType::AuthorizationError)
    }
}

pub fn get_access_token(user_id: &Uuid, secret: &str) -> AccessToken {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id.to_string());
    let token_str = claims.sign_with_key(&key).unwrap();
    AccessToken {
        access_token: token_str,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Authenticated {
    pub user_id: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = ApiErrorType;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let config = req.rocket().state::<RustyConfig>().unwrap();
        let res = Authenticated::get(req, &config.rocket_secret);
        match res {
            Ok(authenticated) => request::Outcome::Success(authenticated),
            Err(error) => request::Outcome::Failure((error.get_status(), error)),
        }
    }
}

impl Authenticated {
    pub fn get(req: &Request<'_>, secret: &str) -> Result<Authenticated, ApiErrorType> {
        let headers: Vec<&str> = req.headers().get("Authorization").collect();
        if headers.len() != 1 {
            return Err(ApiErrorType::AuthenticationError);
        }
        let header = headers[0].to_owned();
        let tokens: Vec<&str> = header.split_whitespace().collect();
        if tokens.len() != 2 || tokens[0] != "Bearer" {
            return Err(ApiErrorType::AuthenticationError);
        }

        let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        let claims: BTreeMap<String, String> = tokens[1].verify_with_key(&key).unwrap();
        let user_id = Uuid::parse_str(claims.get("user_id").unwrap()).unwrap();
        Ok(Authenticated { user_id })
    }
}
