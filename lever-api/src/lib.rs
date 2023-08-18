pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn setup_database_pool() -> Pool<ConnectionManager<diesel::SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    pool
}