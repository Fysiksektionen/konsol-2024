use leptos::prelude::*;

/// Empty string (the default when unset) is treated the same as "not
/// configured" — matching the previous screen-frontend's dev fallback,
/// where an empty `VITE_GOOGLE_CALENDAR_API_KEY` made `Calendar.tsx` show
/// mock events instead of hitting the Google Calendar API.
#[server]
pub async fn get_calendar_api_key() -> Result<Option<String>, ServerFnError> {
    let key = std::env::var("GOOGLE_CALENDAR_API_KEY").unwrap_or_default();
    Ok(if key.is_empty() { None } else { Some(key) })
}
