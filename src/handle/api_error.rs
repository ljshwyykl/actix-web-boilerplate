// src/api_error.rs
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub code: u16,
    pub msg: String,
}

impl ApiError {
    pub fn new(code: u16, msg: String) -> ApiError {
        ApiError { code, msg }
    }
}

impl Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.msg.clone(),
            false => {
                error!("{}", self.msg);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
