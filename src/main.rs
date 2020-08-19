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
use crate::api::users::static_rocket_route_info_for_user_read;
use crate::api::users::static_rocket_route_info_for_user_create;
use crate::api::users::static_rocket_route_info_for_user_update;
use crate::api::users::static_rocket_route_info_for_user_delete;

use crate::api::active_code::static_rocket_route_info_for_active_code_read;
use crate::api::active_code::static_rocket_route_info_for_active_code_create;
use crate::api::active_code::static_rocket_route_info_for_active_code_delete;

use crate::api::check_code::static_rocket_route_info_for_check_code_read;
use crate::api::check_code::static_rocket_route_info_for_check_code_create;
use crate::api::check_code::static_rocket_route_info_for_check_code_delete;

use crate::api::task::static_rocket_route_info_for_task_read;
use crate::api::task::static_rocket_route_info_for_task_create;
use crate::api::task::static_rocket_route_info_for_task_update;
use crate::api::task::static_rocket_route_info_for_task_delete;

use crate::api::device::static_rocket_route_info_for_device_read;
use crate::api::device::static_rocket_route_info_for_device_create;
use crate::api::device::static_rocket_route_info_for_device_update;
use crate::api::device::static_rocket_route_info_for_device_delete;


#[database("info")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket_db_api() -> rocket::Rocket {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from("mysql://root:12345678@127.0.0.1/pipe"));
    databases.insert("info", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .port(1122)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(DbConn::fairing())
        .mount("/api/user"
               , routes![user_read, user_create, user_update, user_delete])
        .mount("/api/user/active_code"
               , routes![active_code_read, active_code_create, active_code_delete])
        .mount("/api/user/check_code"
               , routes![check_code_read, check_code_create, check_code_delete])
        .mount("/api/task"
               , routes![task_read, task_create, task_update, task_delete])
        .mount("/api/device"
               , routes![device_read, device_create, device_update, device_delete])
}

fn main() {
    rocket_db_api().launch();
}
