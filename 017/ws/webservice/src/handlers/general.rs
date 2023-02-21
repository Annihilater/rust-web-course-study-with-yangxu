use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    // health_check_response 是共享的不可变的
    let health_check_response = &app_state.health_check_response;
    // 访问 Mutex 字段之前必须先 lock，以防止其他线程同时更新值
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    // 然后更新 visit_count
    *visit_count += 1;
    // 创建 HttpResponse 并返回
    HttpResponse::Ok().json(&response)
}
