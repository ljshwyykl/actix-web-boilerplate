use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    HttpServer::new(|| App::new().service(router::index))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
