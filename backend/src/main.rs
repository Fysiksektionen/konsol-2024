#[macro_use]
extern crate diesel;

use std::fs;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, error, middleware, web, App, HttpServer};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_cors::Cors;
use chrono::{NaiveDate, NaiveTime};
use diesel::{prelude::*, r2d2};
use uuid::Uuid;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod actions;
mod models;
mod schema;
mod fs_helpers;
mod routes;
mod auth;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// The directory where slide images are saved. Will be created if it does not exist. This whole directory gets served as static files.
// No trailing slash
const SLIDE_IMAGE_DIR: &str = "./slide_images";

#[derive(Debug, MultipartForm)]
struct SlideUploadForm {
    caption: Text<String>,
    start: Text<String>,
    end: Text<String>,
    visible: Text<bool>,
    #[multipart(rename = "imageFile")]
    image_file: TempFile,
}

impl SlideUploadForm {
    fn parse_form(self, id: Uuid) -> Result<(models::Slide, TempFile), actix_web::Error> {
        
        // Get the MIME type of the file. This determines the file extension
        let mime = match self.image_file.content_type {
            Some(ref mime) => mime,
            None => {return Err(error::ErrorInternalServerError("No content type found for uploaded file"))},
        };

        Ok((
            models::Slide {
                id: id.into(),
                caption: self.caption.into_inner(),
                // Since the form currently only has date input (but the db stores both date and time), we set the time to 00:00:00
                // To do this, we first parse the date into a NaiveDate, and then add the time with NaiveDate::and_time
                start_date: self.start.into_inner().parse::<NaiveDate>()
                .map_err(error::ErrorInternalServerError)?
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                end_date: self.end.into_inner().parse::<NaiveDate>()
                .map_err(error::ErrorInternalServerError)?
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                active: self.visible.into_inner(),
                filetype: mime.subtype().to_string(),
            },
        
            self.image_file
        ))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Secret key for cookie session store
    let secret_key_string = std::env::var("COOKIE_SECRET_KEY")
        .unwrap_or_else(|_| {
            log::warn!("COOKIE_SECRET_KEY not set; using default insecure key mashed key");
            "hejjakwdjaklwjdlka jwkldjalkwdjalkwjdlkajlkdja klwd231ahwdah widuhalwiudh aliuwhdjlad".to_string()
        });
    let secret_key = Key::from(secret_key_string.as_bytes());
    
    // Check if cookies should be sent only over HTTPS
    let secure_cookies = std::env::var("COOKIE_SECURE")
        .unwrap_or_else(|_| {
            log::warn!("COOKIE_SECURE not set; defaulting to false");
            "false".to_string()
        })
        .to_lowercase() == "true";
    if !secure_cookies {
        log::warn!("COOKIE_SECURE is false; cookies will be able to be sent over HTTP");
    }
    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    pool.get().expect("could not get DB connection for migrations")
        .run_pending_migrations(MIGRATIONS)
        .expect("could not run database migrations");

    // If SLIDE_IMAGE_DIR does not exist, create it
    if !fs::exists(SLIDE_IMAGE_DIR).expect("Unable to check if slide image directory exists") {
        log::info!("Creating slide image directory at {SLIDE_IMAGE_DIR}");
        fs::create_dir_all(SLIDE_IMAGE_DIR)
            .unwrap_or_else(|_| panic!("Unable to create slide image directory at {SLIDE_IMAGE_DIR}"));
    }

    log::info!("saving images at {SLIDE_IMAGE_DIR}");

    log::info!("starting Actix backend at http://0.0.0.0:8080");


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // TODO: restrict cors
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(secure_cookies) // Controlled via COOKIE_SECURE env var; should be true in production
                .build()
                )
            .wrap(cors)
            .service(routes::save_slide)
            .service(routes::get_slides)
            .service(routes::verify_token)
            .service(routes::login_status)
            .service(routes::logout)
            .service(routes::add_user)
            .service(routes::remove_user)
            .service(routes::list_users)
            .service(routes::get_settings)
            .service(actix_files::Files::new("/slides/images",SLIDE_IMAGE_DIR))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be valid path to SQLite DB file")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn test_get_slides_returns_ok() {
        dotenvy::dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool();

        

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(routes::save_slide)
                .service(routes::get_slides),
        )
        .await;

        let req1 = test::TestRequest::get().uri("/api/screen/slides").to_request();
        let res1 = test::call_service(&app, req1).await;
        assert_eq!(res1.status(), StatusCode::OK);
    }
}
