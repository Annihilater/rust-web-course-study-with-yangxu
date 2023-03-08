#[path = "../mod.rs"]
mod wa;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use env_logger::Env;
use routers::app_config;
use std::env;
use wa::{errors, handlers, models, routers};

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // env_logger::from_env(Env::default().default_filter_or("info")).init(); // 初始化日志
    // use of deprecated function `env_logger::from_env`: Prefer `env_logger::Builder::from_env()` instead.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is required");
    println!("Listening on: {}", &host_port);

    // CARGO_MANIFEST_DIR 对应的是 webapp 目录，不是 ws 目录
    // 意思是从 它下面 static 下面的目录寻找静态文件
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(Logger::default()) // 使用日志中间件
            // .wrap(Logger::new("%a %{User-Agent}i")) // 新增日志格式
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
