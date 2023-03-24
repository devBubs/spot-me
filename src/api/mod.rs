use rocket::serde::json::Json;

use crate::model::io::ApiError;

pub mod auth;
pub mod catalog;
pub mod food_logging;

#[catch(500)]
pub fn fatal() -> Json<ApiError> {
    Json(ApiError {
        status_code: 500,
        error: "Not your fault. We will look into it. Sorry!".to_owned(),
    })
}

#[catch(401)]
pub fn not_authenticated() -> Json<ApiError> {
    Json(ApiError {
        status_code: 401,
        error: "The request is not authenticated.".to_owned(),
    })
}

#[catch(403)]
pub fn not_authorized() -> Json<ApiError> {
    Json(ApiError {
        status_code: 403,
        error: "The request is not authorized to perform this action.".to_owned(),
    })
}

#[catch(404)]
pub fn not_found() -> Json<ApiError> {
    Json(ApiError {
        status_code: 404,
        error: "The resource you are looking for does not exist.".to_owned(),
    })
}

#[catch(400)]
pub fn bad_request() -> Json<ApiError> {
    Json(ApiError {
        status_code: 400,
        error: "This is a bad request. Check the API documentation.".to_owned(),
    })
}
