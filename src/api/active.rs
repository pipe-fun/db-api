use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::active_code;

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name = "active_code"]
pub struct ActiveCode {
    code: String,
    owner: String,
}

#[post("/create", format = "json", data = "<new_code>")]
pub fn code_create(conn: DbConn, new_code: Json<ActiveCode>) -> Result<JsonValue, JsonValue> {
    use crate::schema::active_code;
    diesel::insert_into(active_code::table)
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
pub fn code_delete(conn: DbConn, code: String) -> Result<JsonValue, JsonValue> {
    use crate::schema::active_code::dsl::active_code;
    diesel::delete(active_code.find(code))
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}
