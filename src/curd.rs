use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use async_trait::async_trait;
use anyhow::Result;
use actix_web::{Responder, HttpResponse};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Status {
    status: String,
}

impl Status {
    pub fn ok() -> Self {
        Self { status: "ok".into() }
    }

    pub fn err(e: String) -> Self {
        Self { status: e }
    }
}

#[async_trait]
pub trait CURD where Self: Sized + Serialize {
    type KeyType;
    type RequestType;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status>;
    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status>;
    async fn read(pool: &PgPool) -> Result<Vec<Self>>;
    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self>;
    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status>;
}

pub fn deal_result<T: Serialize>(result: Result<T>) -> impl Responder {
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::BadRequest().json(Status::err(e.to_string()))
    }
}
