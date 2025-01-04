// src/services/mod.rs
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::{
    models::Settings,
    errors::SettingsError,
    repositories::SettingsRepository,
};

pub struct SettingsService {
    repository: Arc<dyn SettingsRepository>,
    cache: Arc<RwLock<Option<Settings>>>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            repository,
            cache: Arc::new(RwLock::new(None)),
        }
    }

    async fn ensure_settings_exist(&self) -> Result<Settings, SettingsError> {
        match self.repository.get().await? {
            Some(settings) => Ok(settings),
            None => {
                let default_settings = Settings {
                    id: Uuid::new_v4().to_string(),
                    dark_mode: false,
                    slide_interval: 3000,
                };
                self.repository.save(default_settings.clone()).await?;
                Ok(default_settings)
            }
        }
    }

    pub async fn get_settings(&self) -> Result<Settings, SettingsError> {
        if let Some(settings) = self.cache.read().unwrap().as_ref() {
            return Ok(settings.clone());
        }

        let settings = self.ensure_settings_exist().await?;
        *self.cache.write().unwrap() = Some(settings.clone());
        Ok(settings)
    }

    pub async fn update_settings(&self, settings: Settings) -> Result<(), SettingsError> {
        // Validate
        if settings.slide_interval < 1000 {
            return Err(SettingsError::Validation("Slide interval must be >= 1000ms".into()));
        }
        if settings.id.is_empty() {
            return Err(SettingsError::Validation("ID cannot be empty".into())); 
        }

        // Update
        self.repository.save(settings.clone()).await?;
        *self.cache.write().unwrap() = Some(settings);
        Ok(())
    }

}