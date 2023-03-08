use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use env_logger::Env;
use errors::MyError;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../dbaccess/mod.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init(); // 初始化日志

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'am ok.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080") // 允许 http://localhost:8080
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost") // 允许所有以 localhost 开头的域
            })
            .allowed_methods(vec!["GET", "POST", "DELETE"]) // 允许的方法
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]) // 允许的 headers
            .allowed_header(http::header::CONTENT_TYPE) //允许的 header
            .max_age(3600); // 最长时间是3600秒

        App::new()
            .app_data(shared_data.clone()) // 把 AppState 注册到 app
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            })) //
            .wrap(Logger::default()) // 使用日志中间件
            // .wrap(Logger::new("%a %{User-Agent}i")) // 新增日志格式
            .configure(general_routes) // 配置 app 的路由
            .configure(course_routes)
            .configure(teacher_routes)
            .wrap(cors)
    };

    // 配置 web server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
