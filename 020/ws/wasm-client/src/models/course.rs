// use super::super::log;
use super::super::errors::MyError;
use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
use chrono::NaiveDateTime;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: NaiveDateTime,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// 访问 webservice 来读取课程
pub async fn get_courses_by_teacher(teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors); // 设置允许跨域

    let url = format!("http://localhost:3000/courses/{}", teacher_id);

    // 使用 new_with_str_and_init 发送请求
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;

    // 获取浏览器 dom 里面的 window 对象
    let window = web_sys::window().ok_or("no window exists".to_string())?;
    // 使用 window 的 fetch_with_request api 发送请求，是异步发送
    // 所以前面套了一个 JsFuture::from().await?
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // 确认一下返回的值是不是 Response 的实例
    assert!(resp_value.is_instance_of::<Response>());

    // 将 Response 读取出来
    let resp: Response = resp_value.dyn_into().unwrap();
    // 将其转化为 json 数据，这个 json 在 rust 里面的类型是一个 JsValue，在 errors.rs 文件定义了 from
    let json = JsFuture::from(resp.json()?).await?;

    let courses: Vec<Course> = json.into_serde().unwrap();

    Ok(courses)
}

pub async fn delete_course(teacher_id: i32, course_id: i32) -> () {
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors); // 设置允许跨域

    let url = format!("http://localhost:3000/courses/{}/{}", teacher_id, course_id);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    let _course: Course = json.into_serde().unwrap();
}

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn add_course(name: String, description: String) -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let str_json = format!(
        r#"
        {{
            "teacher_id": 1,
            "name": "{}",
            "description": "{}"
        }}
        "#,
        name, description
    );
    opts.body(Some(&JsValue::from_str(str_json.as_str())));
    let url = "http://localhost:3000/courses/";

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().ok_or("no window exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    Ok(resp.json()?)
}
