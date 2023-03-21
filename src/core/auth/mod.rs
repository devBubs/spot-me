use rocket::http::{Cookie, CookieJar};
use uuid::Uuid;

const USER_ID_COOKIE_NAME: &str = "user_id";

pub fn get_logged_in_user_id(cookies: &CookieJar<'_>) -> Option<Uuid> {
    cookies
        .get_private(USER_ID_COOKIE_NAME)
        .map(|crumb| Uuid::parse_str(crumb.value()).unwrap())
}

pub fn set_logged_in_user_id(cookies: &CookieJar<'_>, user_id: Uuid) {
    cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user_id.to_string()))
}

pub fn unset_logged_in_user_id(cookies: &CookieJar<'_>) {
    cookies.remove_private(Cookie::named(USER_ID_COOKIE_NAME));
}
