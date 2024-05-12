use crate::models::{User, UserNew, UserJson};
use crate::Pool;
use anyhow::Result;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use actix_web::{HttpResponse, Error, http::StatusCode, web};
use serde_json::json;

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
pub async fn get_user_by_id(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let user_id = path.into_inner().0;
    let mut db_connection = pool.get().unwrap();
    match users.filter(id.eq(user_id)).first::<User>(&mut db_connection){
        Ok(_) => {
            let user = users.filter(id.eq(user_id))
                .first::<User>(&mut db_connection)
                .map_err(|err| {
                    actix_web::error::ErrorInternalServerError(err)
                })?;
            Ok(HttpResponse::Ok().json(user))
        },
        Err(_) => {
            let response_json = json!({
                "message": "User not found"
            });
            return Ok(HttpResponse::NotFound().json(response_json));
        }
    }
}

// ROUTE 4
pub async fn delete_user_by_id(pool: web::Data<Pool>, path: web::Path<(i32, )>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let user_id = path.into_inner().0;
    let mut db_connection = pool.get().unwrap();
    match users.filter(id.eq(user_id)).first::<User>(&mut db_connection) {
        Ok(_) => {
            diesel::delete(users.filter(id.eq(user_id)))
                .execute(&mut db_connection)
                .map_err(|err| {
                    actix_web::error::ErrorInternalServerError(err)
                })?;
            let response_json = json!({
                "message": "User deleted successfully"
            });
            Ok(HttpResponse::Ok().json(response_json))
        },
        Err(_) => {
            let response_json = json!({
                "message": "User not found"
            });
            Ok(HttpResponse::NotFound().json(response_json))
        },
    }
}

// ROUTE 5
pub async fn update_user_by_id(pool: web::Data<Pool>, path: web::Path<(i32,)>, user: web::Json<UserJson>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let user_id = path.into_inner().0;
    let mut db_connection = pool.get().unwrap();

    match users.filter(id.eq(user_id)).first::<User>(&mut db_connection) {
        Ok(existing_user) => {
            let new_user = UserNew {
                name: &user.name,
                address: &user.address,
                created_at: &existing_user.created_at
            };

            diesel::update(users)
            .set(&new_user)
            .execute(&mut db_connection)
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(err)
            })?;

            let response_json = json!({
                "message": "User updated"
            });
            Ok(HttpResponse::Ok().json(response_json))
        },
        Err(_) => {
            let response_json = json!({
                "message": "User not found"
            });
            Ok(HttpResponse::NotFound().json(response_json))
        },
    }
}

// ROUTE 6
pub async fn create_user(pool: web::Data<Pool>, user: web::Json<UserJson>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let mut db_connection = pool.get().unwrap();

    match users.filter(name.eq(&user.name)).first::<User>(&mut db_connection){
        Ok(_) => {
            let response_json = json!({
                "message": "User already exists"
            });
            Ok(HttpResponse::BadRequest().json(response_json))
        },
        Err(_) => {
            let new_user = UserNew {
                name: &user.name,
                address: &user.address,
                created_at: &format!("{}", chrono::Local::now().naive_local())
            };

            diesel::dsl::insert_into(users)
            .values(&new_user)
            .execute(&mut db_connection)
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(err)
            })?;

            let response_json = json!({
                "message": "User created"
            });
            Ok(HttpResponse::Ok().json(response_json))
        }
    }
}

async fn list_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let mut db_connection = pool.get().unwrap();
    let result = users.load::<User>(&mut db_connection)?;
    Ok(result)
}