use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
use async_trait::async_trait;
use crate::crud::{CRUD, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Device {
    token: String,
    name: String,
    owner: String
}

impl Responder for Device {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

#[async_trait]
#[allow(unused_variables)]
impl CRUD for Device {
    type KeyType = String;
    type RequestType = Device;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO device (token, name, owner) VALUES ($1, $2, $3)")
            .bind(&r.token)
            .bind(&r.name)
            .bind(&r.owner)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(Status::ok())
    }

    async fn read(pool: &PgPool) -> Result<Vec<Self>> {
        let mut data = vec![];

        let recs = sqlx::query!(
            r#"
                SELECT * FROM device
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            data.push(Device {
                token: rec.token,
                name: rec.name,
                owner: rec.owner,
            });
        }

        Ok(data)
    }

    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM device WHERE token = $1
                "#,
                &key
            )
            .fetch_one(&*pool)
            .await?;

        Ok(Device {
            token: rec.token,
            name: rec.name,
            owner: rec.owner,
        })
    }

    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("UPDATE device SET name = $1 WHERE token = $2")
            .bind(&r.token)
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }

    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("DELETE FROM device WHERE token = $1")
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }
}