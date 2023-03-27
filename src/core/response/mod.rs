use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::Request;

use crate::model::io::{ApiErrorType, ApiResponse, ApiResult};

pub fn respond<T>(data: T) -> ApiResponse<T> {
    Ok(Json(ApiResult::of(data)))
}

impl<T> ApiResult<T> {
    pub fn of(data: T) -> Self {
        Self { data }
    }
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
