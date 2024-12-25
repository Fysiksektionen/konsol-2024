// This code is based on this example from Actix Web: 
// https://github.com/actix/examples/tree/master/databases/diesel
// I only kept the routes and tests here for example purposes and they are not 
// supposed to be part of the final project.

#[macro_use]
extern crate diesel;

use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use diesel::{prelude::*, r2d2, sql_types::Json};
use models::FrontendSettings;
use serde::ser::Impossible;
use uuid::Uuid;

mod actions;
mod models;
mod schema;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;


fn initialize_settings(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    FrontendSettings::initialize_from_db(&mut conn);
}


#[get("/settings")]
async fn get_settings() -> impl Responder {
    match FrontendSettings::get_state() {
        Some(settings) => HttpResponse::Ok().json(settings), // Return JSON response
        None => HttpResponse::NotFound().body("Settings not initialized"),
    }
}


#[post("/settings")]
async fn update_settings(
    pool: web::Data<DbPool>,
    new_settings: web::Json<FrontendSettings>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    match FrontendSettings::update_state(&mut conn, new_settings.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Settings updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = initialize_db_pool();
    initialize_settings(&pool); // Initialize the settings

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_settings) // Include route for settings
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn user_routes() {
        dotenvy::dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                //.service(get_user)
                //.service(add_user),
        )
        .await;

        // send something that isn't a UUID to `get_user`
        let req = test::TestRequest::get().uri("/user/123").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"UUID parsing failed"),
            "unexpected body: {body:?}",
        );

        // try to find a non-existent user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", Uuid::nil()))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"No user found"),
            "unexpected body: {body:?}",
        );

        /* // create new user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(models::NewUser::new("Test user"))
            .to_request();
        let res: models::User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test user");

        // get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", res.id))
            .to_request();
        let res: models::User = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test user");

        // delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(res.id)))
            .execute(&mut pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table"); */
    }
}
