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

pub fn get_id(_client: &Client, _provider: OauthProvider, _uid: String) -> Option<Uuid> {
    // TODO: redesign user table to make provider+uid lookup efficient
    // TODO: implement this using the gsi
    todo!()
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
    input: OauthUserInfo,
    provider: OauthProvider,
    uid: String,
) -> User {
    let mut connected_accounts = HashMap::new();
    connected_accounts.insert(provider.to_string(), S(uid));
    let result = client
        .update_item()
        .return_values(ReturnValue::AllNew)
        .table_name(TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression(
            "SET first_name=:first_name, last_name=:last_name, email=:email, connected_accounts=:connected_accounts",
        )
        .expression_attribute_values(":first_name", S(input.first_name))
        .expression_attribute_values(":last_name", S(input.last_name))
        .expression_attribute_values(":email", S(input.email))
        .expression_attribute_values(":connected_accounts", M(connected_accounts))
        .send()
        .await
        .unwrap()
        .attributes()
        .unwrap()
        .clone();
    convert(&result)
}

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
        .update_expression("SET first_name=:first_name, last_name=:last_name, email=:email")
        .expression_attribute_values(":first_name", S(input.first_name))
        .expression_attribute_values(":last_name", S(input.last_name))
        .expression_attribute_values(":email", S(input.email))
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
    }
}

fn convert(output: &HashMap<String, AttributeValue>) -> User {
    let id = Uuid::parse_str(output["id"].as_s().unwrap()).unwrap();
    let first_name = output["first_name"].as_s().unwrap().to_owned();
    let last_name = output["last_name"].as_s().unwrap().to_owned();
    let email = output["email"].as_s().unwrap().to_owned();
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
    }
}
