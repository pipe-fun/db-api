use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDateTime, NaiveTime};
use crate::crud::{CRUD, Status};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Task {
    id: i32,
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewTask {
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

impl Responder for Task {
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
impl CRUD for Task {
    type KeyType = i32;
    type RequestType = NewTask;

    async fn create(r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        sqlx::query(r#"INSERT INTO task
        (name, succeed_count, failed_count, last_executed, owner, command, execute_time, device_token, active)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#)
            .bind(&r.name)
            .bind(&r.succeed_count)
            .bind(&r.failed_count)
            .bind(&r.last_executed)
            .bind(&r.owner)
            .bind(&r.command)
            .bind(&r.execute_time)
            .bind(&r.device_token)
            .bind(&r.active)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(Status::ok())
    }

    async fn read(pool: &PgPool) -> Result<Vec<Self>> {
        let mut data = vec![];

        let recs = sqlx::query!(
            r#"
                SELECT * FROM task
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            data.push(Task {
                id: rec.id,
                name: rec.name,
                succeed_count: rec.succeed_count,
                failed_count: rec.failed_count,
                last_executed: rec.last_executed.unwrap(),
                owner: rec.owner,
                command: rec.command,
                execute_time: rec.execute_time,
                device_token: rec.device_token,
                active: rec.active
            });
        }

        Ok(data)
    }

    async fn read_by_key(key: Self::KeyType, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM task WHERE id = $1
                "#,
                key
            )
            .fetch_one(&*pool)
            .await?;

        Ok(Task {
            id: rec.id,
            name: rec.name,
            succeed_count: rec.succeed_count,
            failed_count: rec.failed_count,
            last_executed: rec.last_executed.unwrap(),
            owner: rec.owner,
            command: rec.command,
            execute_time: rec.execute_time,
            device_token: rec.device_token,
            active: rec.active
        })
    }

    async fn update(key: Self::KeyType, r: Self::RequestType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query(r#"UPDATE task SET
        name = $1, succeed_count = $2, failed_count = $3, last_executed = $4,
        command = $5, execute_time = $6, device_token = $7, active = $8 WHERE id = $9"#)
            .bind(&r.name)
            .bind(&r.succeed_count)
            .bind(&r.failed_count)
            .bind(&r.last_executed)
            .bind(&r.command)
            .bind(&r.execute_time)
            .bind(&r.device_token)
            .bind(&r.active)
            .bind(key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }

    async fn delete(key: Self::KeyType, pool: &PgPool) -> Result<Status> {
        let mut tx = pool.begin().await?;
        let rows = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(key)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        let s = if rows > 0 { Status::ok() } else { Status::err("not found".into()) };
        Ok(s)
    }
}