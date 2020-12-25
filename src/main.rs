#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use env_logger;
use modules::user;


mod router;
mod modules;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("main starting up");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    HttpServer::new(|| App::new()
        .service(router::index)
        .configure(user::router::init_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
