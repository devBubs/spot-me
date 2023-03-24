use crate::model::OauthUserInfo;

pub async fn get_access_token(_auth_code: String) -> String {
    "bleh".to_owned()
}
pub async fn get_uid(_access_token: &str) -> String {
    "id".to_owned()
}
pub fn get_info(_access_token: &str) -> OauthUserInfo {
    OauthUserInfo {
        first_name: "name".to_owned(),
        last_name: "name".to_owned(),
        email: "n@n.com".to_owned(),
    }
}
