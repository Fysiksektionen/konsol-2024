use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::settings)]
pub struct Settings {
    pub id: String,
    pub dark_mode: bool,
    pub slide_interval: i32,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder::default()
    }

    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("ID cannot be empty".into());
        }
        if self.slide_interval < 1000 {
            return Err("Slide interval must be >= 1000ms".into());
        }
        Ok(())
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

    pub fn build(self) -> Result<Settings, String> {
        let settings = Settings {
            id: uuid::Uuid::new_v4().to_string(),
            dark_mode: self.dark_mode.unwrap_or(false),
            slide_interval: self.slide_interval.unwrap_or(3000),
        };
        
        settings.validate()?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_builder_valid() {
        let settings = Settings::builder()
            .dark_mode(true)
            .slide_interval(2000)
            .build()
            .unwrap();
        
        assert!(settings.dark_mode);
        assert_eq!(settings.slide_interval, 2000);
    }

    #[test]
    fn test_settings_validation_invalid_interval() {
        let result = Settings::builder()
            .slide_interval(500)
            .build();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("1000ms"));
    }
}