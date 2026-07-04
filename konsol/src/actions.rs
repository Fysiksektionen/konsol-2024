#![cfg(feature = "ssr")]

use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{self, PermissionLevel, Slide, User};
use diesel::SqliteConnection;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_slides(conn: &mut SqliteConnection) -> Result<Vec<Slide>, DbError> {
    use crate::schema::slides::dsl::*;

    let all_slides = slides.load::<Slide>(conn)?;

    Ok(all_slides)
}

pub fn insert_slide(conn: &mut SqliteConnection, slide: models::Slide) -> Result<models::Slide, DbError> {
    use crate::schema::slides::dsl::*;

    diesel::insert_into(slides).values(&slide).execute(conn)?;

    Ok(slide)
}

/// Insert a user into the database and return it
pub fn insert_user(conn: &mut SqliteConnection, user: User) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    // Check if the user already exists
    let existing_user = users
        .filter(email.eq(user.email.clone()))
        .first::<User>(conn)
        .optional()?;

    // If the user already exists, return it instead
    if let Some(user) = existing_user {
        return Ok(user);
    }

    diesel::insert_into(users)
        .values(&user)
        .on_conflict(email)
        .do_nothing()
        .execute(conn)?;

    Ok(user)
}

pub fn remove_user(conn: &mut SqliteConnection, user_id: &str) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;

    Ok(())
}

pub fn pop_slide(conn: &mut SqliteConnection, uuid: &Uuid) -> Result<Slide, DbError> {
    use crate::schema::slides::dsl::*;

    // Get slide
    let slide = slides.find(uuid.to_string()).first(conn)?;

    // Delete slide
    diesel::delete(slides.filter(id.eq(uuid.to_string()))).execute(conn)?;

    Ok(slide)
}

/// Takes an `UpdateSlide` and updates every field that is Some(_)
#[allow(dead_code)]
pub fn update_slide(conn: &mut SqliteConnection, update_slide: models::UpdateSlide) -> Result<(), DbError> {
    use crate::schema::slides::dsl::*;

    diesel::update(slides.find(update_slide.id)).set(update_slide).execute(conn)?;

    Ok(())
}

pub fn get_all_users(conn: &mut SqliteConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let all_users = users.load::<User>(conn)?;

    Ok(all_users)
}

/// Check a user against the database.
/// Returns None if the user does not exist, otherwise a PermissionLevel
pub fn check_user(conn: &mut SqliteConnection, email_str: &str) -> Result<Option<PermissionLevel>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(email_str.to_owned()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user.map(|u| PermissionLevel::from(u.admin)))
}

pub fn get_settings(conn: &mut SqliteConnection) -> Result<models::Settings, DbError> {
    use crate::schema::settings::dsl::*;

    let setting = settings.first::<models::Settings>(conn)?;

    Ok(setting)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use super::*;

    fn new_test_slide() -> Slide {
        Slide {
            id: Uuid::new_v4().to_string(),
            caption: "Test Slide".to_string(),
            start_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .expect("Invalid datetime"),
            end_date: NaiveDateTime::parse_from_str("2015-09-08 20:01:04", "%Y-%m-%d %H:%M:%S")
                .expect("Invalid datetime"),
            active: true,
            filetype: "png".to_string(),
        }
    }

    fn init_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        let slides_migration = include_str!("../migrations/2024-12-12-181858_create_slides/up.sql");
        diesel::sql_query(slides_migration).execute(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_get_all_slides() {
        let mut conn = init_test_db();
        let slides = get_all_slides(&mut conn).unwrap();
        assert!(slides.is_empty());
    }

    #[test]
    fn test_insert_slide() {
        let mut conn = init_test_db();
        let slide = new_test_slide();

        let inserted_slide = insert_slide(&mut conn, slide.clone()).unwrap();
        assert_eq!(inserted_slide, slide);

        let slides = get_all_slides(&mut conn).unwrap();
        assert_eq!(slides.len(), 1);
        assert_eq!(slides[0], slide);
    }

    #[test]
    fn test_pop_slide() {
        let mut conn = init_test_db();
        let slide = new_test_slide();

        insert_slide(&mut conn, slide.clone()).unwrap();
        let popped_slide = pop_slide(&mut conn, &Uuid::parse_str(&slide.id).unwrap()).unwrap();
        assert_eq!(popped_slide, slide);

        let slides = get_all_slides(&mut conn).unwrap();
        assert!(slides.is_empty());
    }

    #[test]
    fn test_update_slide() {
        let mut conn = init_test_db();
        let mut slide = new_test_slide();

        insert_slide(&mut conn, slide.clone()).unwrap();

        let update_slide_struct = models::UpdateSlide {
            id: &slide.id,
            caption: Some("New Title"),
            start_date: None,
            end_date: None,
            active: None,
            filetype: None,
        };

        update_slide(&mut conn, update_slide_struct).unwrap();

        let updated_slide = get_all_slides(&mut conn).unwrap().pop().unwrap();
        slide.caption = "New Title".to_string();

        assert_eq!(updated_slide, slide);
    }
}
