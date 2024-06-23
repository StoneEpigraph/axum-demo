use std::env;
use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite, SqlitePool};
use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect(" DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.expect("can't connect to database");
    pool
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Counter {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub value: i32,
    pub setp: i32,
    pub input_step: bool,
    pub sequence: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Debug, Serialize)]
pub struct CounterRecord {
    pub id: i32,
    pub counter_id: i32,
    pub step: i32,
    pub begin: i32,
    pub end: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}