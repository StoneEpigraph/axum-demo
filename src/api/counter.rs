use axum::extract::State;
use axum::Json;
use sqlx::{Pool, Sqlite};

use crate::api::ApiError;
use crate::api::jwt::Uid;
use crate::db::Counter;

pub async fn list(
    // claims: Claims,
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<Counter>>, ApiError> {
    let counters = sqlx::query_as::<_, Counter>("select * from counters where user_id = ? order by sequence desc")
        .bind(user_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(counters))
}
