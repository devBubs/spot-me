pub mod google_oauth;

use std::{fmt, str::FromStr};

use crate::model::{OauthProvider, OauthUserInfo};

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

pub async fn get_access_token(auth_code: String, provider: &OauthProvider) -> String {
    match provider {
        OauthProvider::GOOGLE => google_oauth::get_access_token(auth_code).await,
        _ => panic!("Invalid provider"),
    }
}
pub async fn get_uid(access_token: &str, provider: &OauthProvider) -> String {
    match provider {
        OauthProvider::GOOGLE => google_oauth::get_uid(access_token).await,
        _ => panic!("Invalid provider"),
    }
}
pub async fn get_info(access_token: &str, provider: &OauthProvider) -> OauthUserInfo {
    match provider {
        OauthProvider::GOOGLE => google_oauth::get_info(access_token),
        _ => panic!("Invalid provider"),
    }
}
