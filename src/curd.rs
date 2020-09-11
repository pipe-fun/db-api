use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use async_trait::async_trait;
use anyhow::Result;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Status {
    status: String,
}

#[async_trait]
pub trait CURD where Self: Sized + Serialize {
    type KeyType;
    type RequestType;

    async fn create(pool: &PgPool) -> Status;
    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Status;
    async fn read(pool: &PgPool) -> Result<Vec<Self>>;
    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Vec<Self>>;
    async fn delete(key: Self::KeyType, pool: &PgPool) -> Status;
}