use crate::models::Settings;
use leptos::prelude::*;

#[server]
pub async fn get_settings() -> Result<Settings, ServerFnError> {
    use crate::actions;
    use crate::db;

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::get_settings(&mut conn))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}
