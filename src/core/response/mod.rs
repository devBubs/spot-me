use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::Request;
use serde::{Deserialize, Serialize};

pub type ApiResponse<T> = Result<Json<ApiResult<T>>, ApiErrorType>;

pub fn respond<T>(data: T) -> ApiResponse<T> {
    Ok(Json(ApiResult::of(data)))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResult<T> {
    pub data: T,
}

impl<T> ApiResult<T> {
    pub fn of(data: T) -> Self {
        Self { data }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub status_code: u32,
    pub error: String,
}

pub enum ApiErrorType {
    AuthenticationError,
    AuthorizationError,
    ValidationError,
    UnknownError,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiErrorType {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            ApiErrorType::AuthenticationError => Status::Unauthorized.respond_to(req),
            ApiErrorType::AuthorizationError => Status::Forbidden.respond_to(req),
            ApiErrorType::ValidationError => Status::BadRequest.respond_to(req),
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
