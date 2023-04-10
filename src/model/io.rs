use super::CatalogItemType;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

pub type ApiResponse<T> = Result<Json<ApiResult<T>>, ApiErrorType>;

#[derive(Debug)]
pub enum ApiErrorType {
    AuthenticationError,
    AuthorizationError,
    ValidationError,
    UnknownError,
}

impl ApiErrorType {
    pub fn get_status(&self) -> Status {
        match self {
            ApiErrorType::AuthenticationError => Status::Unauthorized,
            ApiErrorType::AuthorizationError => Status::Forbidden,
            ApiErrorType::ValidationError => Status::BadRequest,
            ApiErrorType::UnknownError => Status::InternalServerError,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub status_code: u32,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpsertRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogItemUpsertRequest {
    pub name: String,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub item_type: CatalogItemType,
}
