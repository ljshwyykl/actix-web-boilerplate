use super::api_error::ApiError;
use super::api_success::ApiSuccess;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

pub async fn send<T: Serialize>(t: Result<T, ApiError>) -> impl Responder {
    match t {
        Ok(message) => HttpResponse::Ok().json(ApiSuccess::new(message)),
        Err(err) => HttpResponse::Ok().json(err),
    }
}
