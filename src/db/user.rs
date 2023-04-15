use crate::model::{io::UserUpsertRequest, OauthProvider, OauthUserInfo, User};
use aws_sdk_dynamodb::{
    model::{
        AttributeValue::{self, M, S},
        ReturnValue,
    },
    Client,
};
use std::collections::HashMap;
use uuid::Uuid;

const TABLE_NAME: &str = "spot-me.user";
// TODO: move to db
const LOGGED_OUT_USER_ID: &str = "a9b8101a-8222-4004-a4ba-70a9e2d6a974";
const LOGGED_OUT_USER_FIRST_NAME: &str = "Annonymous";
const LOGGED_OUT_USER_LAST_NAME: &str = "Guest";
const LOGGED_OUT_EMAIL: &str = "dummy@dummy.com";

pub async fn get_id(client: &Client, provider: &OauthProvider, uid: &str) -> Option<User> {
    // TODO: redesign user table to make provider+uid lookup efficient
    // TODO: implement this using the gsi
    let users = fetch_all(client).await;
    let matched_users = users
        .into_iter()
        .filter(|user| {
            if let Some(current_uid) = user.connected_accounts.get(&provider.to_string()) {
                return uid.eq(current_uid);
            } else {
                return false;
            }
        })
        .collect::<Vec<User>>();
    if matched_users.len() > 1 {
        panic!("Duplicate users found!!!");
    }
    matched_users.first().cloned()
}

pub async fn get_or_create(
    client: &Client,
    provider: &OauthProvider,
    user_info: &OauthUserInfo,
) -> User {
    let uid = user_info.uid.as_ref();
    if let Some(user) = get_id(client, provider, uid).await {
        return user;
    }
    create(client, Uuid::new_v4(), user_info, provider, uid).await
}

pub async fn fetch_all(client: &Client) -> Vec<User> {
    let output = client.scan().table_name(TABLE_NAME).send().await.unwrap();
    let items = output.items().unwrap();
    items.to_vec().iter().map(|v| convert(v)).collect()
}

pub async fn fetch(client: &Client, id: Uuid) -> User {
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

pub async fn create(
    client: &Client,
    id: Uuid,
    input: &OauthUserInfo,
    provider: &OauthProvider,
    uid: &str,
) -> User {
    let mut connected_accounts = HashMap::new();
    connected_accounts.insert(provider.to_string(), S(uid.to_owned()));
    let result = client
        .update_item()
        .return_values(ReturnValue::AllNew)
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression(
            "SET first_name=:first_name, last_name=:last_name, email=:email, connected_accounts=:connected_accounts, picture=:picture",
        )
        .expression_attribute_values(":first_name", S(input.first_name.clone()))
        .expression_attribute_values(":last_name", S(input.last_name.clone()))
        .expression_attribute_values(":email", S(input.email.clone()))
        .expression_attribute_values(":connected_accounts", M(connected_accounts))
        .expression_attribute_values(":picture", S(input.picture.clone()))
        .send()
        .await
        .unwrap()
        .attributes()
        .unwrap()
        .clone();
    convert(&result)
}

// TODO: There seems to a be bug with this ddb operation
pub async fn add_connected_account(
    client: &Client,
    id: Uuid,
    provider: OauthProvider,
    uid: String,
) -> User {
    // TODO: handle already connected account case
    let mut connected_accounts = fetch(client, id).await.connected_accounts;
    connected_accounts.insert(provider.to_string(), uid);
    let connected_accounts = connected_accounts
        .iter()
        .map(|(provider, uid)| (provider.to_owned(), S(uid.clone())))
        .collect::<HashMap<String, AttributeValue>>();
    let result = client
        .update_item()
        .return_values(ReturnValue::AllNew)
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression("SET connected_accounts=:connected_accounts")
        .expression_attribute_values("connected_accounts", M(connected_accounts))
        .send()
        .await
        .unwrap()
        .attributes()
        .unwrap()
        .clone();
    convert(&result)
}

pub async fn update(client: &Client, id: Uuid, input: UserUpsertRequest) -> User {
    let result = client
        .update_item()
        .return_values(ReturnValue::AllNew)
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression(
            "SET first_name=:first_name, last_name=:last_name, email=:email, picture=:picture",
        )
        .expression_attribute_values(":first_name", S(input.first_name))
        .expression_attribute_values(":last_name", S(input.last_name))
        .expression_attribute_values(":email", S(input.email))
        .expression_attribute_values(":picture", S(input.picture))
        .send()
        .await
        .unwrap()
        .attributes()
        .unwrap()
        .clone();
    convert(&result)
}

pub fn get_logged_out_user() -> User {
    // TODO: Implement config table and put this there
    User {
        id: Uuid::parse_str(LOGGED_OUT_USER_ID).unwrap(),
        first_name: LOGGED_OUT_USER_FIRST_NAME.to_owned(),
        last_name: LOGGED_OUT_USER_LAST_NAME.to_owned(),
        email: LOGGED_OUT_EMAIL.to_owned(),
        connected_accounts: HashMap::new(),
        picture: "https://e7.pngegg.com/pngimages/416/62/png-clipart-anonymous-person-login-google-account-computer-icons-user-activity-miscellaneous-computer-thumbnail.png".to_owned(),
    }
}

fn convert(output: &HashMap<String, AttributeValue>) -> User {
    let id = Uuid::parse_str(output["id"].as_s().unwrap()).unwrap();
    let first_name = output["first_name"].as_s().unwrap().to_owned();
    let last_name = output["last_name"].as_s().unwrap().to_owned();
    let email = output["email"].as_s().unwrap().to_owned();
    let picture = output["picture"].as_s().unwrap().to_owned();
    let connected_accounts = output["connected_accounts"]
        .as_m()
        .unwrap()
        .iter()
        .map(|(provider, uid)| (provider.to_owned(), uid.as_s().unwrap().to_owned()))
        .collect::<HashMap<_, _>>();
    User {
        id,
        first_name,
        last_name,
        email,
        connected_accounts,
        picture,
    }
}
