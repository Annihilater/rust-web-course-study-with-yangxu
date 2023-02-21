use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 把 .env 文件里面的环境变量读取出来，为什么加 ok() 呢？
    // 因为 dotenv 返回的是一个 result，result 没用的话就会有一个警告，调用 ok() 可以把警告去掉
    // 另外很重要的一点，生产环境中，你不会使用 .env 来放置环境变量，你会在生产环境的系统里来设置环境变量
    // 所以说那个时候这个 .env 文件不存在，它读取加载就会失败，而调用 ok() 就会把 Result<T, E> 转为 Option<T> 类型
    // 这个时候即使失败也会忽略这个失败，所以说这里就这么调用，记住就可以了
    dotenv().ok();

    // 读取环境变量的值
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    // 创建数据库的连接池
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where id = $1"#,
        1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
        })
    }
    println!("Courses = {:?}", courses_list);
    Ok(())
}
