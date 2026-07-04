#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use konsol::app::{shell, App};
    use konsol::db;
    use konsol::fs_helpers::SLIDE_IMAGE_DIR;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;

    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let pool = db::init_pool();
    pool.get()
        .expect("could not get DB connection for migrations")
        .run_pending_migrations(MIGRATIONS)
        .expect("could not run database migrations");

    if !std::fs::exists(SLIDE_IMAGE_DIR).expect("unable to check if slide image directory exists") {
        log::info!("Creating slide image directory at {SLIDE_IMAGE_DIR}");
        std::fs::create_dir_all(SLIDE_IMAGE_DIR)
            .unwrap_or_else(|_| panic!("Unable to create slide image directory at {SLIDE_IMAGE_DIR}"));
    }

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    log::info!("starting konsol at http://{addr}");

    let app = Router::new()
        .nest_service("/screen/slides/images", ServeDir::new(SLIDE_IMAGE_DIR))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
