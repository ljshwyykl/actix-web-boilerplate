####  add logger
```
[dependencies]
log = "0.4.0"
env_logger = "0.8.2"
```

```
///.env
RUST_LOG=rest_api=info,actix=info

```
```
///main.rs
#[macro_use]
extern crate log;


use env_logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ...
    env_logger::init();
     
    info!("main starting up");
    ...
    
}

```


### model、controller、router

```
// struct
src
    modules
        user
            controller.rs
            model.rs
            router.rs
        mod.rs    
    main.rs
    router.rs
```

```
///src/modules/user/model.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}

```

```
///src/modules/user/controller.rs
use actix_web::{get, HttpResponse, Responder};
use super::model::User;



#[get("/users")]
pub async fn all() -> impl Responder {
    HttpResponse::Ok().json(vec![User {
        id: 1,
        email: "tetd@qq.com".to_string(),
    }])
}

```
```
///src/modules/user/router.rs
use super::controller::all;
use actix_web::web;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(all);
}

```

```
///src/modules/user/mod.rs
pub mod model;
pub mod controller;
pub mod router;
```

```
///src/modules/mod.rs
pub mod user;
```


```
/// main.rs

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
        .configure(user::router::register_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

```


```
///cargo.toml

[dependencies]
actix-web = "3"
dotenv = "0.15.0"
log = "0.4.0"
env_logger = "0.8.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

```####  add logger
```
[dependencies]
log = "0.4.0"
env_logger = "0.8.2"
```

```
///.env
RUST_LOG=rest_api=info,actix=info

```
```
///main.rs
#[macro_use]
extern crate log;


use env_logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ...
    env_logger::init();
     
    info!("main starting up");
    ...
    
}

```


### model、controller、router

```
// struct
src
    modules
        user
            controller.rs
            model.rs
            router.rs
        mod.rs    
    main.rs
    router.rs
```

```
///src/modules/user/model.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}

```

```
///src/modules/user/controller.rs
use actix_web::{get, HttpResponse, Responder};
use super::model::User;



#[get("/users")]
pub async fn all() -> impl Responder {
    HttpResponse::Ok().json(vec![User {
        id: 1,
        email: "tetd@qq.com".to_string(),
    }])
}

```
```
///src/modules/user/router.rs
use super::controller::all;
use actix_web::web;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(all);
}

```

```
///src/modules/user/mod.rs
pub mod model;
pub mod controller;
pub mod router;
```

```
///src/modules/mod.rs
pub mod user;
```


```
/// main.rs

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
        .configure(user::router::register_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

```


```
///cargo.toml

[dependencies]
actix-web = "3"
dotenv = "0.15.0"
log = "0.4.0"
env_logger = "0.8.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

```