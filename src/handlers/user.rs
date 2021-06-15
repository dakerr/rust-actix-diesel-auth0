use crate::database::PoolType;
use crate::errors::ApiError;
use crate::models::user::{find, get_all_users, add_single_user, delete_single_user};
use actix_web::web::{block, Data, HttpResponse, Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_user(user_id: Path<i32>, pool: Data<PoolType>) -> Result<HttpResponse, ApiError> {
    Ok(
        block(move || find(&pool, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ApiError::InternalServerError(format!("User not found")))?
    )
}

pub async fn get_users(pool: Data<PoolType>) -> Result<HttpResponse, ApiError> {
    Ok(
        block(move || get_all_users(&pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ApiError::InternalServerError(format!("get users error")))?
    )
}

pub async fn add_user(pool: Data<PoolType>, item: Json<InputUser>) -> Result<HttpResponse, ApiError> {
    Ok(
        block(move || add_single_user(&pool, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| ApiError::InternalServerError(format!("add user error")))?
    )
}

pub async fn delete_user(pool: Data<PoolType>, user_id: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(
        block(move || delete_single_user(&pool, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ApiError::InternalServerError(format!("delete user error")))?
    )
}