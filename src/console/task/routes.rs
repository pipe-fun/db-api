use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;
use crate::console::task::model::{Task, NewTask};
use crate::crud::{CRUD, deal_result};

#[post("/task/create")]
async fn create(new: web::Json<NewTask>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Task::create(new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/task/read")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Task::read(db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/task/read_by_code/{id}")]
async fn read_by_name(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Task::read_by_key(id.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[put("/task/update/{id}")]
async fn update(id: web::Path<i32>, new: web::Json<NewTask>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Task::update(id.into_inner(), new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[delete("/task/delete/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Task::delete(id.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(read);
    cfg.service(read_by_name);
    cfg.service(update);
    cfg.service(delete);
}