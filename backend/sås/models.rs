use serde::{Deserialize, Serialize};
use validator::Validate;
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Queryable, Insertable)]
#[diesel(table_name = crate::schema::settings)]
pub struct Settings {
    #[validate(length(min = 1))]
    pub id: String,
    pub dark_mode: bool,
    #[validate(range(min = 1000))]
    pub slide_interval: i32,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder::default()
    }
}

#[derive(Default)]
pub struct SettingsBuilder {
    dark_mode: Option<bool>,
    slide_interval: Option<i32>,
}

impl SettingsBuilder {
    pub fn dark_mode(mut self, value: bool) -> Self {
        self.dark_mode = Some(value);
        self
    }

    pub fn slide_interval(mut self, value: i32) -> Self {
        self.slide_interval = Some(value);
        self
    }

    pub fn build(self) -> Result<Settings, crate::errors::SettingsError> {
        let settings = Settings {
            id: uuid::Uuid::new_v4().to_string(),
            dark_mode: self.dark_mode.unwrap_or(false),
            slide_interval: self.slide_interval.unwrap_or(3000),
        };
        
        settings.validate().map_err(|e| crate::errors::SettingsError::Validation(e.to_string()))?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_builder() {
        let settings = Settings::builder()
            .dark_mode(true)
            .slide_interval(2000)
            .build()
            .unwrap();
        
        assert!(settings.dark_mode);
        assert_eq!(settings.slide_interval, 2000);
    }
}