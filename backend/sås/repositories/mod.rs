use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::Error as PoolError;
use std::sync::Arc;

use crate::{
    models::Settings,
    errors::SettingsError,
    schema::settings,
    schema::settings::dsl::*,
    DbPool,
};


pub struct DbSettingsRepository {
    pool: Arc<DbPool>,
}

impl DbSettingsRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get(&self) -> Result<Option<Settings>, SettingsError>;
    async fn save(&self, settings: Settings) -> Result<(), SettingsError>;
}

#[async_trait]
impl SettingsRepository for DbSettingsRepository {
    async fn get(&self) -> Result<Option<Settings>, SettingsError> {
        let mut conn = self.pool.get()
            .map_err(|e| SettingsError::ConnectionError(e.to_string()))?;

        settings::table
            .first(&mut conn)
            .optional()
            .map_err(|e| SettingsError::DatabaseError(e.to_string()))
    }

    async fn save(&self, new_settings: Settings) -> Result<(), SettingsError> {
        let mut conn = self.pool.get()
            .map_err(|e| SettingsError::ConnectionError(e.to_string()))?;

        diesel::replace_into(settings::table)
            .values(&new_settings)
            .execute(&mut conn)
            .map_err(|e| SettingsError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
}
