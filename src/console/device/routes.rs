use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;
use crate::console::device::model::Device;
use crate::crud::{CRUD, deal_result};

#[post("/device/create")]
async fn create(new: web::Json<Device>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Device::create(new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/device/read")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Device::read(db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/device/read_by_code/{token}")]
async fn read_by_name(token: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Device::read_by_key(token.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[put("/device/update/{token}")]
async fn update(token: web::Path<String>, new: web::Json<Device>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Device::update(token.into_inner(), new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[delete("/device/delete/{token}")]
async fn delete(token: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Device::delete(token.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(read);
    cfg.service(read_by_name);
    cfg.service(update);
    cfg.service(delete);
}