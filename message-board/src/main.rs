#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

use crate::models::{NewPost, Post};
use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    let url = env::var("DATABASE_URL").expect("找不到資料庫位置");
    SqliteConnection::establish(&url).expect("連線失敗")
}

fn create_post(conn: &SqliteConnection, author: &str, title: &str, body: &str) {
    use self::schema::posts;

    let new_post = NewPost {
        author,
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("新增貼文失敗");
}

fn list_posts(conn: &SqliteConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    posts.load::<Post>(conn).expect("取得貼文列表失敗")
}

fn get_post(conn: &SqliteConnection, id: i32) -> Post {
    use self::schema::posts::dsl::*;

    posts.find(id).first(conn).expect("取得貼文失敗")
}

fn delete_post(conn: &SqliteConnection, id: i32) {
    use self::schema::posts::dsl::*;

    diesel::delete(posts.find(id))
        .execute(conn)
        .expect("刪除貼文失敗");
}

fn main() {
    dotenv().ok();
    let conn = establish_connection();
}
