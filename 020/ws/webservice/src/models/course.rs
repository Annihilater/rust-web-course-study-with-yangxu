use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// use crate::models::course::Course
// course 这个结构体不用来新增作者修改，只用作读取，所以把 Deserialize 删掉
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,             // 课程讲师
    pub id: i32,                     // 课程ID
    pub name: String,                // 课程名称
    pub time: Option<NaiveDateTime>, // 课程时间

    pub description: Option<String>, // 课程描述
    pub format: Option<String>,      // 课程格式
    pub structure: Option<String>,   // 课程结构
    pub duration: Option<String>,    // 课程持续时间
    pub price: Option<i32>,          // 课程价格
    pub language: Option<String>,    // 课程语言
    pub level: Option<String>,       // 课程级别
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// // 因为进来的数据是 json 格式的数据
// // 这里就是针对进来的请求的数据转化为 Course 类型
// // web::Json 和 web::Data 形式很像，web::Json 里面的字段基本上是可以直接调用的
// // 它们两个都是数据提取器，在这里的意思就是可以把请求里面的 json 数据提取为 Course 类型的数据
// impl From<web::Json<Course>> for CreateCourse {
//     fn from(course: web::Json<Course>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }

// 如果 From 转换容易出错，那么可以使用 TryFrom 来实现数据库表结构到 rust 结构体模型之间的转换
impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    // 如果成功则返回 CreateCourse 类型
    // 如果失败则返回 MyError 自定义的错误类型
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
