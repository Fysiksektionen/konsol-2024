use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::sync::Arc;
use serde_json::json;

mod schema;
mod models;
mod actions;

use actions::{SettingsActions, Error};
use models::Settings;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn setup_database() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;
    
    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Migration error: {}", e))?;
    
    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let pool = setup_database()
        .expect("Failed to setup database");
    let settings_actions = Arc::new(SettingsActions::new(Arc::new(pool.clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(settings_actions.clone()))
            .wrap(Logger::default())
            .service(web::scope("/api")
                .route("/settings", web::get().to(get_settings))
                .route("/settings", web::post().to(update_settings)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_settings(
    actions: web::Data<Arc<SettingsActions>>,
) -> HttpResponse {
    match actions.get_settings().await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
}

async fn update_settings(
    actions: web::Data<Arc<SettingsActions>>,
    settings: web::Json<Settings>,
) -> HttpResponse {
    match actions.update_settings(settings.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success" })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))
    }
}