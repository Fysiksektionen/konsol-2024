use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::schema::{settings, slides, users};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::QueryableByName, diesel::Insertable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = slides))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Slide {
    pub id: String,
    pub caption: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub active: bool,
    pub filetype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::QueryableByName, diesel::Insertable, diesel::Selectable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct User {
    pub id: String,
    pub email: String,
    pub admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::QueryableByName, diesel::Insertable, diesel::Selectable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = settings))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Settings {
    pub id: i32,
    pub layout_type: String,
    pub color_mode: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, diesel::AsChangeset)]
#[diesel(table_name = slides)]
pub struct UpdateSlide<'a> {
    pub id: &'a str,
    pub caption: Option<&'a str>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub filetype: Option<&'a str>,
}

/// Permission level for an authenticated user. Crosses the client/server
/// boundary (server function args/results), unlike the DB-facing `User`
/// above whose `admin: bool` is the on-disk representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    User,
    Admin,
}

impl From<bool> for PermissionLevel {
    fn from(admin: bool) -> Self {
        if admin {
            PermissionLevel::Admin
        } else {
            PermissionLevel::User
        }
    }
}

/// The authenticated user, as stored in the session cookie and returned by
/// auth-related server functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub email: String,
    pub permission: PermissionLevel,
}
