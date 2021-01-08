use super::model::{User, UserEntity};
use crate::handle::api_error::{ApiError};
use crate::handle::handle_response::send;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/users")]
async fn all() -> impl Responder {
    let users = User::find_all();
    // success_response(users)

    // let users = User::find_all().await;
    send(users).await

    // let users = User::find_all();
    // ApiSuccess {
    //     code: 100,
    //     msg: "String".to_string(),
    //     data: users,
    // }


    // User::find_all()
    // .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
    // // match User::find_all() {
    //     Ok(message) => success_response(message),
    //     Err(err) => success_response(err),
    // }
    // ApiSuccess {
    //     code: 100,
    //     msg: "String".to_string(),
    //     data: users,
    // }
    // Ok(HttpResponse::Ok().json(ApiSuccess::new(100,"String".to_string(), users)))
}

#[post("/users")]
async fn create(user: web::Json<UserEntity>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
