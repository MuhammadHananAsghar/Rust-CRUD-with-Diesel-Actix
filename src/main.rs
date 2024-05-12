#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = Pool::builder()
    .build(ConnectionManager::new(database_url))
    .unwrap();

    HttpServer::new(move || {
        let database_pool = database_pool.clone();
        App::new()
        .data(database_pool.clone())
        .route("/", web::get().to(routes::root))
        .route("/users", web::get().to(routes::get_users))
        .route("/users/{user_id}", web::get().to(routes::get_user_by_id))
    })
    .bind("localhost:8000")?
    .run()
    .await
}