use diesel::{SqliteConnection, prelude::*, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::schema::settings;
use once_cell::sync::Lazy;
use std::sync::Mutex;


/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = settings)]
pub struct FrontendSettings {
    pub id: String,
    pub dark_mode: bool,
    pub slide_interval: i32,
}

pub static SETTINGS: Lazy<Mutex<Option<FrontendSettings>>> = Lazy::new(|| Mutex::new(None));

// Implementation block for FrontendSettings
impl FrontendSettings {
    /// Initializes the settings from the database.
    /// This should be called once during application startup.
    pub fn initialize_from_db(conn: &mut SqliteConnection) {
        use crate::schema::settings::dsl::*;

        let mut settings_guard = SETTINGS.lock().expect("Failed to lock SETTINGS mutex");

        // Load the first row of the settings table
        let result: Option<FrontendSettings> = settings.first(conn).ok();
        *settings_guard = result; // Update the singleton state
    }

    /// Retrieves a copy of the current settings.
    pub fn get_state() -> Option<FrontendSettings> {
        let settings_guard = SETTINGS.lock().expect("Failed to lock SETTINGS mutex");
        settings_guard.clone() // Return a copy of the current settings
    }

    /// Updates the settings in the database and singleton state.
    pub fn update_state(
        conn: &mut SqliteConnection,
        new_settings: FrontendSettings,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::settings::dsl::*;

        diesel::update(settings)
            .set((
                dark_mode.eq(new_settings.dark_mode),
                slide_interval.eq(new_settings.slide_interval),
            ))
            .execute(conn)?;

        // Update the singleton state
        let mut settings_guard = SETTINGS.lock().expect("Failed to lock SETTINGS mutex");
        *settings_guard = Some(new_settings);

        Ok(())
    }
}
/* 
/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    /// Constructs new user details from name.
    #[cfg(test)]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
} */
