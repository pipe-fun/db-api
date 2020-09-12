use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
use async_trait::async_trait;
use crate::curd::{CRUD, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct ActiveCode {
    code: String,
    owner: String,
}

impl Responder for ActiveCode {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

#[async_trait]
#[allow(unused_variables)]
impl CRUD for ActiveCode {
    type KeyType = String;
    type RequestType = ActiveCode;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO active_code (code, owner) VALUES ($1, $2)")
            .bind(&r.code)
            .bind(&r.owner)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(Status::ok())
    }

    async fn read(pool: &PgPool) -> Result<Vec<Self>> {
        let mut users = vec![];

        let recs = sqlx::query!(
            r#"
                SELECT * FROM active_code
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            users.push(ActiveCode {
                code: rec.code,
                owner: rec.owner
            });
        }

        Ok(users)
    }

    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM active_code WHERE code = $1
                "#,
                &key
            )
            .fetch_one(&*pool)
            .await?;

        Ok(ActiveCode {
            code: rec.code,
            owner: rec.owner
        })
    }

    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        unimplemented!()
    }

    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("DELETE FROM active_code WHERE code = $1")
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }
}