use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::pipe_users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_password: String,
    user_email: String,
    user_registered_time: NaiveDateTime,
    user_recently_login_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name="pipe_users"]
pub struct NewUser {
    user_name: String,
    user_password: String,
    user_email: String,
    user_registered_time: NaiveDateTime,
    user_recently_login_time: NaiveDateTime,
}

#[get("/read")]
pub fn read(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::pipe_users::dsl::pipe_users;
    pipe_users.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[post("/create", format = "json", data = "<new_user>")]
pub fn create(conn: DbConn, new_user: Json<NewUser>) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users;
    diesel::insert_into(pipe_users::table)
        .values(&new_user.into_inner())
        .execute(&conn.0)
        .map_err(|_| {
            json!({"status": "err"})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[delete("/delete/<id>")]
pub fn delete(conn: DbConn, id: i32) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users::dsl::pipe_users;
    diesel::delete(pipe_users.find(id))
        .execute(&conn.0)
        .map_err(|_| {
            json!({"status": "err"})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[put("/update/<id>", data = "<new_user>")]
pub fn update(conn: DbConn, id: i32, new_user: Json<NewUser>) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users::dsl::pipe_users;
    diesel::update(pipe_users.find(id))
        .set(&new_user.into_inner())
        .execute(&conn.0)
        .map_err(|_| {
            json!({"status": "err"})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}