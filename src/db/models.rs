use super::schema::task;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub done: i32,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: i32,
}
