use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;
use crate::user::users::model::User;
use crate::curd::{CURD, deal_result};

#[post("/user/create")]
async fn create(new_user: web::Json<User>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(new_user.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/user/read")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::read(db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/user/read_by_name/{name}")]
async fn read_by_name(name: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::read_by_key(name.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[put("/user/update/{name}")]
async fn update(name: web::Path<String>, new: web::Json<User>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::update(name.into_inner(), new.into_inner(),db_pool.get_ref()).await;
    deal_result(result)
}

#[delete("/user/delete/{name}")]
async fn delete(name: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::delete(name.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(read);
    cfg.service(read_by_name);
    cfg.service(update);
    cfg.service(delete);
}