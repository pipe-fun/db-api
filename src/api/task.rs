use diesel::prelude::*;
use chrono::{NaiveTime, NaiveDateTime};
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::task;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    id: i32,
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name = "task"]
pub struct NewTask {
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

#[get("/read")]
pub fn task_read(conn: DbConn) -> Result<Json<Vec<Task>>, String> {
    use crate::schema::task::dsl::task;
    task.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[post("/create", format = "json", data = "<new_task>")]
pub fn task_create(conn: DbConn, new_task: Json<NewTask>) -> Result<JsonValue, JsonValue> {
    use crate::schema::task;
    diesel::insert_into(task::table)
        .values(&new_task.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[delete("/delete/<id>")]
pub fn task_delete(conn: DbConn, id: i32) -> Result<JsonValue, JsonValue> {
    use crate::schema::task::dsl::task;
    diesel::delete(task.find(id))
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[put("/update/<id>", data = "<new_task>")]
pub fn task_update(conn: DbConn, id: i32, new_task: Json<NewTask>) -> Result<JsonValue, JsonValue> {
    use crate::schema::task::dsl::task;
    diesel::update(task.find(id))
        .set(&new_task.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}