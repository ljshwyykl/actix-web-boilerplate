use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    info!("router index");
    HttpResponse::Ok().body("Hello world!11")
}