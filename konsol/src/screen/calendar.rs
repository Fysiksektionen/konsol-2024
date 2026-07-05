//! Calendar widget shown in the mixed layout, alternating with the
//! slideshow every 15s. Ported from the previous screen-frontend's
//! `Calendar.tsx`: fetches upcoming events from the Google Calendar public
//! API, falling back to a few mock events when no API key is configured
//! (same behavior as local dev before).

use leptos::prelude::*;

use crate::api::calendar::get_calendar_api_key;

#[cfg(feature = "hydrate")]
const CALENDAR_ID: &str = "fysiksektionen.se_0187vbmdcivl8mtio142e23cas@group.calendar.google.com";

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub summary: String,
    pub location: Option<String>,
    /// RFC3339 datetime for timed events.
    pub date_time: Option<String>,
    /// Plain `YYYY-MM-DD` date for all-day events.
    pub all_day_date: Option<String>,
}

#[component]
pub fn Calendar() -> impl IntoView {
    let config = Resource::new(|| (), |_| get_calendar_api_key());
    let (events, set_events) = signal(Vec::<CalendarEvent>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(false);

    Effect::new(move |_| {
        if let Some(Ok(api_key)) = config.get() {
            start_polling(api_key, set_events, set_loading, set_error);
        }
    });

    view! {
        <div class="calendar-component">
            <h2 class="calendar-title">"Kalender"</h2>
            {move || {
                if loading.get() {
                    view! { <p class="calendar-loading">"Laddar kalender..."</p> }.into_any()
                } else if error.get() {
                    view! { <p class="calendar-error">"Kunde inte hämta kalender"</p> }.into_any()
                } else {
                    let evs = events.get();
                    if evs.is_empty() {
                        view! { <p class="no-events">"Inga kommande händelser"</p> }.into_any()
                    } else {
                        view! {
                            <div class="events-list">
                                {evs.into_iter().map(|e| view! { <EventCard event=e/> }).collect_view()}
                            </div>
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
}

#[component]
fn EventCard(event: CalendarEvent) -> impl IntoView {
    let is_all_day = event.all_day_date.is_some();
    let start = event
        .date_time
        .clone()
        .or_else(|| event.all_day_date.clone())
        .unwrap_or_default();
    let date_label = format_date(&start);
    let time_label = format_time(&start);

    view! {
        <div class="event-card">
            <div class="event-date">
                <span class="date-main">{date_label}</span>
                {(!is_all_day).then(|| view! { <span class="event-time">{time_label}</span> })}
            </div>
            <div class="event-details">
                <h3 class="event-summary">{event.summary.clone()}</h3>
                {event.location.clone().map(|loc| view! { <p class="event-location">{loc}</p> })}
            </div>
        </div>
    }
}

#[cfg(not(feature = "hydrate"))]
fn format_date(_iso: &str) -> String {
    String::new()
}

#[cfg(not(feature = "hydrate"))]
fn format_time(_iso: &str) -> String {
    String::new()
}

#[cfg(not(feature = "hydrate"))]
fn start_polling(
    _api_key: Option<String>,
    _set_events: WriteSignal<Vec<CalendarEvent>>,
    _set_loading: WriteSignal<bool>,
    _set_error: WriteSignal<bool>,
) {
    // Events are fetched client-side only, same as the previous
    // screen-frontend (a pure client-side React component).
}

#[cfg(feature = "hydrate")]
fn format_date(iso: &str) -> String {
    use wasm_bindgen::JsValue;

    let date = js_sys::Date::new(&JsValue::from_str(iso));
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &"weekday".into(), &"short".into());
    let _ = js_sys::Reflect::set(&opts, &"day".into(), &"numeric".into());
    let _ = js_sys::Reflect::set(&opts, &"month".into(), &"short".into());
    date.to_locale_date_string("sv-SE", &opts).into()
}

#[cfg(feature = "hydrate")]
fn format_time(iso: &str) -> String {
    use wasm_bindgen::JsValue;

    let date = js_sys::Date::new(&JsValue::from_str(iso));
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &"hour".into(), &"2-digit".into());
    let _ = js_sys::Reflect::set(&opts, &"minute".into(), &"2-digit".into());
    date.to_locale_time_string_with_options("sv-SE", &opts).into()
}

#[cfg(feature = "hydrate")]
fn start_polling(
    api_key: Option<String>,
    set_events: WriteSignal<Vec<CalendarEvent>>,
    set_loading: WriteSignal<bool>,
    set_error: WriteSignal<bool>,
) {
    let fetch_once = move || {
        let api_key = api_key.clone();
        leptos::task::spawn_local(async move {
            match api_key {
                None => set_events.set(mock_events()),
                Some(key) => match hydrate_impl::fetch_events(&key).await {
                    Ok(evs) => {
                        set_error.set(false);
                        set_events.set(evs);
                    }
                    Err(e) => {
                        leptos::logging::error!("Calendar error: {e}");
                        set_error.set(true);
                    }
                },
            }
            set_loading.set(false);
        });
    };
    fetch_once();
    crate::client_util::set_interval(fetch_once, 600_000);
}

#[cfg(feature = "hydrate")]
fn mock_events() -> Vec<CalendarEvent> {
    use wasm_bindgen::JsValue;

    let offset_iso = |offset_ms: f64| -> String {
        let date = js_sys::Date::new(&JsValue::from_f64(js_sys::Date::now() + offset_ms));
        date.to_iso_string().into()
    };
    let offset_date_only = |offset_ms: f64| -> String {
        offset_iso(offset_ms).split('T').next().unwrap_or_default().to_string()
    };

    vec![
        CalendarEvent {
            summary: "Pub".to_string(),
            location: Some("Konsulatet".to_string()),
            date_time: Some(offset_iso(3_600_000.0)),
            all_day_date: None,
        },
        CalendarEvent {
            summary: "Sektionsmöte".to_string(),
            location: Some("T-Centralen".to_string()),
            date_time: Some(offset_iso(86_400_000.0)),
            all_day_date: None,
        },
        CalendarEvent {
            summary: "Tentamen i Kvantfysik".to_string(),
            location: Some("FA32".to_string()),
            date_time: None,
            all_day_date: Some(offset_date_only(172_800_000.0)),
        },
    ]
}

#[cfg(feature = "hydrate")]
mod hydrate_impl {
    use super::CalendarEvent;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct RawEventsResponse {
        items: Option<Vec<RawEvent>>,
    }

    #[derive(Deserialize)]
    struct RawEvent {
        summary: String,
        location: Option<String>,
        start: RawEventTime,
    }

    #[derive(Deserialize)]
    struct RawEventTime {
        #[serde(rename = "dateTime")]
        date_time: Option<String>,
        date: Option<String>,
    }

    pub async fn fetch_events(api_key: &str) -> Result<Vec<CalendarEvent>, String> {
        let now: String = js_sys::Date::new_0().to_iso_string().into();
        let calendar_id: String = js_sys::encode_uri_component(super::CALENDAR_ID).into();
        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events?key={api_key}&timeMin={now}&singleEvents=true&orderBy=startTime&maxResults=10"
        );
        let resp = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let parsed: RawEventsResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(parsed
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|e| CalendarEvent {
                summary: e.summary,
                location: e.location,
                date_time: e.start.date_time,
                all_day_date: e.start.date,
            })
            .collect())
    }
}
