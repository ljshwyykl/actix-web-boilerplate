use actix_web::{App, HttpServer};

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(router::index))
        .bind("127.0.0.1:8282")?
        .run()
        .await
}
