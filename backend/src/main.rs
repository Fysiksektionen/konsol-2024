#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

use rocket::{fairing::AdHoc, Build, Rocket};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", routes![index])
}
