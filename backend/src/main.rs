use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::sync::Arc;
use std::net::TcpListener;

mod models;
mod schema;
mod repositories;
mod services;
mod handlers;
mod errors;

use crate::repositories::DbSettingsRepository;
use crate::services::SettingsService;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Run migrations with better error handling
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Migration error: {}", e))?;
    
    // Verify table exists
    let table_exists = diesel::dsl::sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='settings'"
    )
    .execute(connection)
    .is_ok();

    if !table_exists {
        return Err("Settings table not created properly".into());
    }

    Ok(())
}

fn find_available_port() -> std::io::Result<u16> {
    for port in 8000..9000 {
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return Ok(port);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AddrInUse,
        "No available ports found"
    ))
}

fn setup_database() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)?;
    
    // Run migrations
    let mut conn = pool.get()?;
    run_migrations(&mut conn)?;
    
    Ok(pool)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Setup database
    log::info!("Setting up database...");
    let pool = setup_database()?;
    
    // Initialize services
    let repository = Arc::new(DbSettingsRepository::new(Arc::new(pool.clone())));
    let settings_service = Arc::new(SettingsService::new(repository));

    // Find available port
    let port = find_available_port()?;
    log::info!("Starting server at http://localhost:{}", port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(settings_service.clone()))
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(
                web::scope("/api")
                    .configure(handlers::config)
            )
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .shutdown_timeout(5)
    .run()
    .await?;

    Ok(())
}