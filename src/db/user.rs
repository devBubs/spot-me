use crate::model::{io::UserUpsertRequest, OauthProvider, OauthUserInfo, User};
use aws_sdk_dynamodb::{
    model::{
        AttributeValue::{self, M, S},
        ReturnValue, TransactWriteItem, Update,
    },
    Client,
};
use std::collections::HashMap;
use uuid::Uuid;

const USER_TABLE_NAME: &str = "spot-me.user";
const UID_TO_USER_TABLE_NAME: &str = "spot-me.uid_to_user";

pub async fn get_or_create(
    client: &Client,
    provider: &OauthProvider,
    user_info: &OauthUserInfo,
) -> Uuid {
    let uid = user_info.uid.as_ref();
    if let Some(id) = get_id(client, provider, uid).await {
        fetch(client, id).await.id
    } else {
        create(client, Uuid::new_v4(), user_info, provider, uid).await
    }
}

pub async fn fetch_all(client: &Client) -> Vec<User> {
    let output = client
        .scan()
        .table_name(USER_TABLE_NAME)
        .send()
        .await
        .unwrap();
    let items = output.items().unwrap();
    items.to_vec().iter().map(|v| convert(v)).collect()
}

pub async fn fetch(client: &Client, id: Uuid) -> User {
    let results = client
        .get_item()
        .table_name(USER_TABLE_NAME)
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
) -> Uuid {
    let mut connected_accounts = HashMap::new();
    connected_accounts.insert(provider.to_string(), S(uid.to_owned()));
    let user_create = Update::builder()
        .table_name(USER_TABLE_NAME)
        .key("id", S(id.to_string()))
        .update_expression(
            "SET first_name=:first_name, last_name=:last_name, email=:email, connected_accounts=:connected_accounts, picture=:picture",
        )
        .expression_attribute_values(":first_name", S(input.first_name.clone()))
        .expression_attribute_values(":last_name", S(input.last_name.clone()))
        .expression_attribute_values(":email", S(input.email.clone()))
        .expression_attribute_values(":connected_accounts", M(connected_accounts))
        .expression_attribute_values(":picture", S(input.picture.clone()))
        .build();
    let uid_to_user_update = Update::builder()
        .table_name(UID_TO_USER_TABLE_NAME)
        .key("provider", S(provider.to_string()))
        .key("uid", S(uid.to_owned()))
        .update_expression("SET user_id=:user_id")
        .expression_attribute_values(":user_id", S(id.to_string()))
        .build();
    let user_create = TransactWriteItem::builder().update(user_create).build();
    let uid_to_user_update = TransactWriteItem::builder()
        .update(uid_to_user_update)
        .build();
    client
        .transact_write_items()
        .set_transact_items(Some(vec![user_create, uid_to_user_update]))
        .send()
        .await
        .unwrap();
    id
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
        .table_name(USER_TABLE_NAME)
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
        .table_name(USER_TABLE_NAME)
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

async fn get_id(client: &Client, provider: &OauthProvider, uid: &str) -> Option<Uuid> {
    if let Some(item) = client
        .get_item()
        .table_name(UID_TO_USER_TABLE_NAME)
        .key("provider", S(provider.to_string()))
        .key("uid", S(uid.to_owned()))
        .send()
        .await
        .unwrap()
        .item()
        .clone()
    {
        let id = item["user_id"].as_s().unwrap();
        Uuid::parse_str(id).ok()
    } else {
        None
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
