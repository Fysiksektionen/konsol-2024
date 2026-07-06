use crate::models::Settings;
use leptos::prelude::*;

#[server(prefix = "/konsol/api")]
pub async fn get_settings() -> Result<Settings, ServerFnError> {
    use crate::actions;
    use crate::db;

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::get_settings(&mut conn))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(prefix = "/konsol/api")]
pub async fn update_settings(
    layout_type: String,
    live_mode: bool,
    live_slides_url: String,
) -> Result<Settings, ServerFnError> {
    use crate::actions;
    use crate::db;

    crate::session::require_auth().await?;

    if !matches!(layout_type.as_str(), "mixed" | "fullscreen_slideshow") {
        return Err(ServerFnError::new(format!("unknown layout type: {layout_type}")));
    }

    let live_slides_url = live_slides_url.trim().to_string();
    if !live_slides_url.is_empty() && !live_slides_url.starts_with("https://docs.google.com/presentation/") {
        return Err(ServerFnError::new(
            "live slides URL must be a Google Slides link (https://docs.google.com/presentation/...)",
        ));
    }

    let mut conn = db::get_conn()?;
    // The settings row is a singleton (id CHECK'd to 1); color_mode isn't
    // editable here yet, so carry the stored value over.
    let current = tokio::task::spawn_blocking(move || actions::get_settings(&mut conn))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let new = Settings {
        id: 1,
        layout_type,
        color_mode: current.color_mode,
        live_mode,
        live_slides_url,
    };

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::upsert_settings(&mut conn, new))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}
