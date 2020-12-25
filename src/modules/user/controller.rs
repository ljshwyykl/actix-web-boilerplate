use actix_web::{get, HttpResponse, Responder};
use super::model::User;



#[get("/users")]
pub async fn all() -> impl Responder {
    HttpResponse::Ok().json(vec![User {
        id: 1,
        email: "tetd@qq.com".to_string(),
    }])
}
