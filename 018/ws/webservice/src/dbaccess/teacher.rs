use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!(r#"select id, name, picture_url, profile from teacher"#)
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher where id = $1"#,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name.clone(),
        picture_url: r.picture_url.clone(),
        profile: r.profile.clone(),
    })
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    // 如果没有查到任何记录就会返回一个 sqlx 的错误，通过 map_err 可以将这个错误，转化为自定义的错误

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"insert into teacher (name, picture_url, profile) values ($1, $2, $3) returning id, name, picture_url, profile"#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile,
    )
    .fetch_one(pool)
    .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    /*
    1. 先根据teacher_id查询teacher
    2. 再将需要修改的东西在查询到的基础上修改
    3. 再将修改好的结构存入数据库，做更新
     */
    let row = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher where id = $1"#,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            Some(name)
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            Some(picture_url)
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            Some(profile)
        } else {
            row.profile
        },
    };

    let update_row = sqlx::query!(
        r#"
        update teacher
        set name = $1, picture_url = $2, profile = $3
        where id = $4
        returning id, name, picture_url, profile
        "#,
        temp.name,
        temp.picture_url,
        temp.profile,
        temp.id
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name,
        picture_url: r.picture_url,
        profile: r.profile,
    })
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    Ok(update_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("delete from teacher where id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()))?;

    Ok(format!("Delete {:?} record", row))
}
