use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, // 数据库里对应的类型是 serial 是主键
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// 不需要序列化，所以没有 Serialize
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    // 这几个字段都是可修改或者不修改的，所以都是 Option 的
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// 不需要序列化，所以没有 Serialize
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    // 这几个字段都是可修改或者不修改的，所以都是 Option 的
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(new_teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}
