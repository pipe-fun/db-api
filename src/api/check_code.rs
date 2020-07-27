use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::check_code;

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name = "check_code"]
pub struct CheckCode {
    code: i32,
    owner: String,
}

#[get("/read")]
pub fn check_code_read(conn: DbConn) -> Result<Json<Vec<CheckCode>>, String> {
    use crate::schema::check_code::dsl::check_code;
    check_code.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[post("/create", format = "json", data = "<new_code>")]
pub fn check_code_create(conn: DbConn, new_code: Json<CheckCode>) -> Result<JsonValue, JsonValue> {
    use crate::schema::check_code;
    diesel::insert_into(check_code::table)
        .values(&new_code.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[delete("/delete/<code>")]
pub fn check_code_delete(conn: DbConn, code: i32) -> Result<JsonValue, JsonValue> {
    use crate::schema::check_code::dsl::check_code;
    diesel::delete(check_code.find(code))
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}
