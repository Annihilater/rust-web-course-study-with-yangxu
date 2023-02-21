use crate::handlers::{course::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    // 使用 web::scope 先定义了一个作用域 courses
    // 换句话说 courses 就是一套资源的根路径，在它下面可以定义各种各样的资源
    // 现在定义的就是 / 的资源，请求方法是 post 形式
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_courses_detail),
            )
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_course_details),
            ),
    );
}
