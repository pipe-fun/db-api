use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::DbConn;
use crate::schema::device;

#[derive(Serialize, Deserialize, Queryable)]
#[derive(Insertable, AsChangeset)]
#[table_name = "device"]
pub struct Device {
    token: String,
    owner: String
}

#[get("/read")]
pub fn device_read(conn: DbConn) -> Result<Json<Vec<Device>>, String> {
    use crate::schema::device::dsl::device;
    device.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying".into()
    }).map(Json)
}

#[post("/create", format = "json", data = "<new_device>")]
pub fn device_create(conn: DbConn, new_device: Json<Device>) -> Result<JsonValue, JsonValue> {
    use crate::schema::device;
    diesel::insert_into(device::table)
        .values(&new_device.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[delete("/delete/<token>")]
pub fn device_delete(conn: DbConn, token: String) -> Result<JsonValue, JsonValue> {
    use crate::schema::device::dsl::device;
    diesel::delete(device.find(token))
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}

#[put("/update/<token>", data = "<new_device>")]
pub fn device_update(conn: DbConn, token: String, new_device: Json<Device>) -> Result<JsonValue, JsonValue> {
    use crate::schema::device::dsl::device;
    diesel::update(device.find(token))
        .set(&new_device.into_inner())
        .execute(&conn.0)
        .map_err(|e| {
            json!({"status": e.to_string()})
        })
        .map(|_| {
            json!({"status": "ok"})
        })
}
