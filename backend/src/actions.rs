use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::Error as PoolError;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::{
    models::Settings,
    schema::settings,
    DbPool,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Settings not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
}

pub struct SettingsActions {
    pool: Arc<DbPool>,
    cache: Arc<RwLock<Option<Settings>>>,
}

impl SettingsActions {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            pool,
            cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_settings(&self) -> Result<Settings, Error> {
        // Check cache first
        if let Some(settings) = self.cache.read().unwrap().as_ref() {
            return Ok(settings.clone());
        }

        // Get from database or create default
        let mut conn = self.pool.get()
            .map_err(|e| Error::Connection(e.to_string()))?;

        let settings = match settings::table
            .first::<Settings>(&mut conn)
            .optional()
            .map_err(Error::Database)? {
                Some(s) => s,
                None => {
                    let default_settings = Settings {
                        id: Uuid::new_v4().to_string(),
                        dark_mode: false,
                        slide_interval: 3000,
                    };
                    diesel::insert_into(settings::table)
                        .values(&default_settings)
                        .execute(&mut conn)
                        .map_err(Error::Database)?;
                    default_settings
                }
            };

        // Update cache
        *self.cache.write().unwrap() = Some(settings.clone());
        Ok(settings)
    }

    pub async fn update_settings(&self, new_settings: Settings) -> Result<(), Error> {
        // Validate
        if new_settings.slide_interval < 1000 {
            return Err(Error::Validation("Slide interval must be >= 1000ms".into()));
        }

        let mut conn = self.pool.get()
            .map_err(|e| Error::Connection(e.to_string()))?;

        diesel::replace_into(settings::table)
            .values(&new_settings)
            .execute(&mut conn)
            .map_err(Error::Database)?;

        // Update cache
        *self.cache.write().unwrap() = Some(new_settings);
        Ok(())
    }
}