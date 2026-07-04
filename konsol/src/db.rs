#![cfg(feature = "ssr")]

use diesel::r2d2;
use diesel::SqliteConnection;
use std::sync::OnceLock;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub type DbConn = r2d2::PooledConnection<r2d2::ConnectionManager<SqliteConnection>>;

static POOL: OnceLock<DbPool> = OnceLock::new();

pub fn init_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be valid path to SQLite DB file");
    POOL.set(pool.clone()).expect("init_pool called more than once");
    pool
}

pub fn get_conn() -> Result<DbConn, leptos::prelude::ServerFnError> {
    let pool = POOL.get().expect("DB pool not initialized");
    pool.get().map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))
}
