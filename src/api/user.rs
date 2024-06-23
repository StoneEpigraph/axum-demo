use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use crate::api::ApiError;
use crate::api::jwt::{AuthError, Claims};
use crate::db::User;

#[derive(Deserialize)]
pub struct LoginPayload {
    code: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn login(State(pool): State<Pool<Sqlite>>, Json(payload): Json<LoginPayload>,) -> Result<Json<AuthBody>, ApiError> {
    let wx_user = wx_login(payload.code).await?;

    let user = sqlx::query_as::<_, User>("select * from users where openid = ?")
        .bind(&wx_user.openid)
        .fetch_one(&pool)
        .await;
    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("insert into users(openid, session_key) values(?, ?)")
                .bind(&wx_user.openid)
                .bind(&wx_user.session_key)
                .execute(&pool)
                .await?;
            sqlx::query_as::<_, User>("select * from users where openid = ?")
                .bind(&wx_user.openid)
                .fetch_one(&pool)
                .await?
        },
        Err(e) => return Err(ApiError::from(e)),
    };

    let claims = Claims::new(user.id.to_string());
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

#[derive(Deserialize, Default)]
pub struct WxUser {
    pub openid: String,
    pub session_key: String,
}

async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    Ok(WxUser::default())
}