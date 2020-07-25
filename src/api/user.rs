use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::pipe_users;

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name = "pipe_users"]
pub struct User {
    user_name: String,
    user_password: String,
    user_email: String,
    active: bool,
}

#[get("/read")]
pub fn user_read(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::pipe_users::dsl::pipe_users;
    pipe_users.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[post("/create", format = "json", data = "<new_user>")]
pub fn user_create(conn: DbConn, new_user: Json<User>) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users;
    diesel::insert_into(pipe_users::table)
        .values(&new_user.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[delete("/delete/<user_name>")]
pub fn user_delete(conn: DbConn, user_name: String) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users::dsl::pipe_users;
    diesel::delete(pipe_users.find(user_name))
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[put("/update/<user_name>", data = "<new_user>")]
pub fn user_update(conn: DbConn, user_name: String, new_user: Json<User>) -> Result<JsonValue, JsonValue> {
    use crate::schema::pipe_users::dsl::pipe_users;
    diesel::update(pipe_users.find(user_name))
        .set(&new_user.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}