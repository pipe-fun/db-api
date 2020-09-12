use actix_web::{delete, get, post, web, Responder};
use sqlx::PgPool;
use crate::user::check_code::model::CheckCode;
use crate::crud::{CRUD, deal_result};

#[post("/check_code/create")]
async fn create(new: web::Json<CheckCode>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = CheckCode::create(new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/check_code/read")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = CheckCode::read(db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/check_code/read_by_code/{code}")]
async fn read_by_name(code: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = CheckCode::read_by_key(code.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[delete("/check_code/delete/{code}")]
async fn delete(code: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = CheckCode::delete(code.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(read);
    cfg.service(read_by_name);
    cfg.service(delete);
}