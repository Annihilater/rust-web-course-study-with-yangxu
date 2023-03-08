use actix_web::{error, http::StatusCode, HttpResponse, Result};
// use actix_web::Result;
use serde::Serialize;
use std::fmt;

// NotFound 变体在项目中并没有使用到，那么编译的时候编译器就会发出警告
// 如果不想让编译器发出这个警告，就在前面加上 #[allow(dead_code)] 即可，表示允许死代码
#[allow(dead_code)]
#[derive(Clone, Serialize, Debug)]
pub enum MyError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for MyError {}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::ActixError(_msg) | MyError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
