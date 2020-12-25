####  创建项目

```
cargo new actix-web-boilerplate
```


#### 添加actix-web
``` 
///  cargo.tmol

[dependencies]
actix-web = "3"
dotenv = "0.15.0"
env_logger = "0.8.2"
```

修改 **src/main.rs** 和 **src/router.rs**

```
/// main.rs
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

```

添加 **.env**

```
/// .env 
HOST=127.0.0.1
PORT=5000
```

```
/// router.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!11")
}
```


#### 启动运行

```
cargo run 
```

然后访问 [http://localhost:5000/](http://localhost:5000/)


#### cargo-watch

添加 **cargo-watch**
```
cargo install cargo-watch
```

运行
```
cargo watch -x 'run'
```