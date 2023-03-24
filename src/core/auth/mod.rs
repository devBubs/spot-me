use rocket::http::{Cookie, CookieJar};
use uuid::Uuid;

use crate::model::io::ApiErrorType;

const USER_ID_COOKIE_NAME: &str = "user_id";
const ADMIN_USER_IDS: &[&str] = &["r2's ID", "c2's ID"];

pub fn get_logged_in_user_id(cookies: &CookieJar<'_>) -> Result<Uuid, ApiErrorType> {
    if let Some(id) = cookies
        .get_private(USER_ID_COOKIE_NAME)
        .map(|crumb| Uuid::parse_str(crumb.value()).unwrap())
    {
        Ok(id)
    } else {
        Err(ApiErrorType::AuthenticationError)
    }
}

pub fn set_logged_in_user_id(cookies: &CookieJar<'_>, user_id: Uuid) -> Result<(), ApiErrorType> {
    assert_logged_out(cookies)?;
    cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user_id.to_string()));
    Ok(())
}

pub fn unset_logged_in_user_id(cookies: &CookieJar<'_>) -> Result<(), ApiErrorType> {
    assert_logged_in(cookies)?;
    cookies.remove_private(Cookie::named(USER_ID_COOKIE_NAME));
    Ok(())
}

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

pub fn assert_logged_out(cookies: &CookieJar<'_>) -> Result<(), ApiErrorType> {
    if let Some(_) = cookies
        .get_private(USER_ID_COOKIE_NAME)
        .map(|crumb| Uuid::parse_str(crumb.value()).unwrap())
    {
        Err(ApiErrorType::ValidationError)
    } else {
        Ok(())
    }
}

pub fn assert_logged_in(cookies: &CookieJar<'_>) -> Result<(), ApiErrorType> {
    if let Some(_) = cookies
        .get_private(USER_ID_COOKIE_NAME)
        .map(|crumb| Uuid::parse_str(crumb.value()).unwrap())
    {
        Ok(())
    } else {
        Err(ApiErrorType::ValidationError)
    }
}
