#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod schema;
mod api;

use rocket::Config;
use rocket::config::Environment;
use std::collections::HashMap;
use rocket::config::Value;
use crate::api::user::static_rocket_route_info_for_user_read;
use crate::api::user::static_rocket_route_info_for_user_create;
use crate::api::user::static_rocket_route_info_for_user_update;
use crate::api::user::static_rocket_route_info_for_user_delete;

use crate::api::active::static_rocket_route_info_for_code_read;
use crate::api::active::static_rocket_route_info_for_code_create;
use crate::api::active::static_rocket_route_info_for_code_delete;

#[database("info")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket_db_api() -> rocket::Rocket {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from("mysql://root:12345678@localhost/info"));
    databases.insert("info", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .port(1122)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(DbConn::fairing())
        .mount("/api/user", routes![user_read, user_create, user_update, user_delete])
        .mount("/api/user/active_code", routes![code_read, code_create, code_delete])
}

fn main() {
    rocket_db_api().launch();
}
