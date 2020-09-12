use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
use async_trait::async_trait;
use crate::curd::{CRUD, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    user_name: String,
    user_password: String,
    user_email: String,
    active: bool,
}

impl Responder for User {
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
impl CRUD for User {
    type KeyType = String;
    type RequestType = User;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO users (user_name, user_password, user_email, active) VALUES ($1, $2, $3, $4)")
            .bind(&r.user_name)
            .bind(&r.user_password)
            .bind(&r.user_email)
            .bind(r.active)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(Status::ok())
    }

    async fn read(pool: &PgPool) -> Result<Vec<Self>> {
        let mut users = vec![];

        let recs = sqlx::query!(
            r#"
                SELECT * FROM users
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            users.push(User {
                user_name: rec.user_name,
                user_password: rec.user_password,
                user_email: rec.user_email,
                active: rec.active,
            });
        }

        Ok(users)
    }

    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM users WHERE user_name = $1
                "#,
                &key
            )
            .fetch_one(&*pool)
            .await?;

        Ok(User {
            user_name: rec.user_name,
            user_password: rec.user_password,
            user_email: rec.user_email,
            active: rec.active,
        })
    }

    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("UPDATE users SET user_password = $1, active = $2 WHERE user_name = $3")
            .bind(&r.user_password)
            .bind(r.active)
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }

    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(&key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }
}