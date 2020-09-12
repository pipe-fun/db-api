use actix_web::{delete, get, post, web, Responder};
use sqlx::PgPool;
use crate::user::active_code::model::ActiveCode;
use crate::curd::{CRUD, deal_result};

#[post("/active_code/create")]
async fn create(new: web::Json<ActiveCode>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = ActiveCode::create(new.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/active_code/read")]
async fn read(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = ActiveCode::read(db_pool.get_ref()).await;
    deal_result(result)
}

#[get("/active_code/read_by_code/{code}")]
async fn read_by_name(code: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = ActiveCode::read_by_key(code.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

#[delete("/active_code/delete/{code}")]
async fn delete(code: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = ActiveCode::delete(code.into_inner(), db_pool.get_ref()).await;
    deal_result(result)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(read);
    cfg.service(read_by_name);
    cfg.service(delete);
}