use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("http://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();

    print!("post: http://localhost:3000/teachers/");

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_picture_url", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave" {
        ctx.insert("error", "Dave already exists!");

        ctx.insert("current_name", &params.name);
        ctx.insert("current_picture_url", &params.picture_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url" : &params.picture_url,
            "profile" : &params.profile
        });

        let awc_client = awc::Client::default();

        let res = awc_client
            .post("http://localhost:3000/teachers")
            .send_json(&new_teacher) // 使用 json 格式发送数据
            .await
            .unwrap()
            .body() // 把返回的 body 取出来
            .await?;

        // 使用 &std::str::from_utf8 将返回的 body 转成字符串切片
        // 然后再从字符串切片转成 TeacherResponse 类型，然后显示 teacher id
        let teacher_response: TeacherResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = format!("Congratutlations! Your Id is: {}.", teacher_response.id);
        // s = format!(
        //     "Congratutlations! Your Id is: {}.",
        //     &std::str::from_utf8(&res)?
        // );
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
