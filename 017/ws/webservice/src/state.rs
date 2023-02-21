// use super::models::Course;
use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,

    // 数据库连接池放在 AppState 里就可以在多线程里共享的使用数据库连接池了
    pub db: PgPool,
}
