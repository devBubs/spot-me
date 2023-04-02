pub mod google_oauth;

use crate::{
    config::RustyConfig,
    model::{OauthProvider, OauthUserInfo},
};

use rocket::request::FromParam;
use std::{fmt, str::FromStr};

impl FromStr for OauthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GOOGLE" => Ok(OauthProvider::GOOGLE),
            "GITHUB" => Ok(OauthProvider::GITHUB),
            "MICROSOFT" => Ok(OauthProvider::MICROSOFT),
            "FACEBOOK" => Ok(OauthProvider::FACEBOOK),
            _ => Err(format!("'{}' is not a valid value for OAUTH_PROVIDERS", s)),
        }
    }
}

impl<'a> FromParam<'a> for OauthProvider {
    type Error = String;

    #[inline(always)]
    fn from_param(param: &'a str) -> Result<OauthProvider, Self::Error> {
        OauthProvider::from_str(param.to_uppercase().as_str())
    }
}

impl fmt::Display for OauthProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OauthProvider::GOOGLE => write!(f, "GOOGLE"),
            OauthProvider::GITHUB => write!(f, "GITHUB"),
            OauthProvider::MICROSOFT => write!(f, "MICROSOFT"),
            OauthProvider::FACEBOOK => write!(f, "FACEBOOK"),
        }
    }
}

pub async fn get_access_token(
    auth_code: String,
    provider: &OauthProvider,
    config: &RustyConfig,
) -> String {
    match provider {
        OauthProvider::GOOGLE => {
            let redirect_uri = config.google_redirection_url.as_str();
            let client_id = config.google_client_id.as_str();
            let client_secret = config.google_client_secret.as_str();
            google_oauth::get_access_token(auth_code, redirect_uri, client_id, client_secret).await
        }
        _ => panic!("Invalid provider"),
    }
}
pub async fn get_info(access_token: &str, provider: &OauthProvider) -> OauthUserInfo {
    match provider {
        OauthProvider::GOOGLE => google_oauth::get_info(access_token).await,
        _ => panic!("Invalid provider"),
    }
}
