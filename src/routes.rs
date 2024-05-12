use crate::models::{User, UserNew,UserJson};
use crate::Pool;
use anyhow::Result;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;
use actix_web::{HttpResponse, Error, http::StatusCode, web};

// ROUTE 1
pub async fn root() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello World"))
}

// ROUTE 2
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error>{
    Ok(list_users(pool)
    .await
    .map(|users| HttpResponse::Ok().json(users))
    .map_err(|_| actix_web::error::InternalError::new(
        "Failed to get users",
        StatusCode::INTERNAL_SERVER_ERROR,
    ))?)
}

// ROUTE 3
pub async fn get_user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(find_user_by_id(pool, user_id)
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| actix_web::error::InternalError::new(
        "Failed to get user by id",
        StatusCode::INTERNAL_SERVER_ERROR
    ))?)
}

async fn find_user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut db_connection = pool.get().unwrap();
    let result = users
        .filter(id.eq(*user_id))
        .first::<User>(&mut db_connection)?;
    Ok(vec![result])
}

async fn list_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut db_connection = pool.get().unwrap();
    let result = users.load::<User>(&mut db_connection)?;
    Ok(result)
}