//! SL (Stockholm public transit) departure board widget.
//!
//! Matches the previous screen-frontend's architecture: departures are
//! fetched directly from SL's public transport API by the browser, not
//! proxied through our own backend. Polls every 5s, same as before.

use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use serde::Deserialize;

/// Sites tracked on the board. Site ids found via
/// <https://www.trafiklab.se/api/trafiklab-apis/sl/stop-lookup> (generally
/// the last 4 digits of SiteId).
pub const TRACKED_SITES: &[u32] = &[
    9204, // tekniska högskolan
    9600, // östra station
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlTransportMode {
    Train,
    Metro,
    Bus,
    Tram,
    Ferry,
    Ship,
    Taxi,
}

impl TryFrom<&str> for SlTransportMode {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "TRAIN" => Self::Train,
            "METRO" => Self::Metro,
            "BUS" => Self::Bus,
            "TRAM" => Self::Tram,
            "FERRY" => Self::Ferry,
            "SHIP" => Self::Ship,
            "TAXI" => Self::Taxi,
            other => return Err(format!("Unexpected transport mode {other}")),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlLineGroup {
    Train,
    RedMetro,
    GreenMetro,
    BlueMetro,
    Bus,
    BlueBus,
    CityLine,
    RoslagenLine,
}

impl TryFrom<&str> for SlLineGroup {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "Pendeltåg" => Self::Train,
            "Tunnelbanans röda linje" => Self::RedMetro,
            "Tunnelbanans gröna linje" => Self::GreenMetro,
            "Tunnelbanans blå linje" => Self::BlueMetro,
            "Buss" => Self::Bus,
            "Blåbuss" => Self::BlueBus,
            "Spårväg City" => Self::CityLine,
            "Roslagsbanan" => Self::RoslagenLine,
            other => return Err(format!("Unexpected line group {other}")),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SlDeparture {
    pub site_id: u32,
    pub transport_mode: SlTransportMode,
    pub line_group: SlLineGroup,
    pub line_id: i64,
    pub direction_code: i64,
    pub stop_point_name: String,
    pub stop_point_designation: String,
    pub destination: String,
    pub via: Option<String>,
    pub line_designation: String,
    pub display_time: String,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Deserialize)]
struct RawDeparturesResponse {
    departures: Vec<RawDeparture>,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Deserialize)]
struct RawDeparture {
    destination: String,
    via: Option<String>,
    direction_code: i64,
    display: String,
    journey: RawJourney,
    stop_point: RawStopPoint,
    line: RawLine,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Deserialize)]
struct RawJourney {
    #[allow(dead_code)]
    id: i64,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Deserialize)]
struct RawStopPoint {
    name: String,
    designation: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Deserialize)]
struct RawLine {
    id: i64,
    designation: String,
    transport_mode: String,
    group_of_lines: Option<String>,
}

#[cfg(feature = "hydrate")]
fn parse_departure(raw: RawDeparture, site_id: u32) -> Result<SlDeparture, String> {
    let transport_mode = SlTransportMode::try_from(raw.line.transport_mode.as_str())?;
    let line_group = match raw.line.group_of_lines.as_deref() {
        Some(s) => SlLineGroup::try_from(s)?,
        None if transport_mode == SlTransportMode::Bus => SlLineGroup::Bus,
        None => return Err("Expected line group".to_string()),
    };
    let _ = raw.journey; // parity with the fields the previous TS type carried, unused in rendering

    Ok(SlDeparture {
        site_id,
        transport_mode,
        line_group,
        line_id: raw.line.id,
        direction_code: raw.direction_code,
        stop_point_name: raw.stop_point.name,
        stop_point_designation: raw.stop_point.designation.unwrap_or_default(),
        destination: raw.destination,
        via: raw.via,
        line_designation: raw.line.designation,
        display_time: raw.display,
    })
}

#[cfg(not(feature = "hydrate"))]
pub fn start_polling(_set_departures: WriteSignal<Vec<SlDeparture>>, _set_last_update: WriteSignal<Option<String>>) {
    // No-op during SSR: SL departures are fetched client-side only, same as
    // the previous screen-frontend (a pure client-side React app).
}

#[cfg(feature = "hydrate")]
pub fn start_polling(set_departures: WriteSignal<Vec<SlDeparture>>, set_last_update: WriteSignal<Option<String>>) {
    leptos::task::spawn_local(fetch_all(set_departures, set_last_update));
    crate::client_util::set_interval(
        move || {
            leptos::task::spawn_local(fetch_all(set_departures, set_last_update));
        },
        5_000,
    );
}

#[cfg(feature = "hydrate")]
async fn fetch_all(set_departures: WriteSignal<Vec<SlDeparture>>, set_last_update: WriteSignal<Option<String>>) {
    let mut all = Vec::new();
    for &site_id in TRACKED_SITES {
        let url = format!("https://transport.integration.sl.se/v1/sites/{site_id}/departures");
        let Ok(resp) = gloo_net::http::Request::get(&url).send().await else {
            leptos::logging::error!("Failed to fetch data from site with id {site_id}");
            continue;
        };
        let Ok(parsed) = resp.json::<RawDeparturesResponse>().await else {
            leptos::logging::error!("Failed to parse departures response for site {site_id}");
            continue;
        };
        for raw in parsed.departures {
            match parse_departure(raw, site_id) {
                Ok(d) => all.push(d),
                Err(e) => leptos::logging::error!("{e}"),
            }
        }
    }
    set_departures.set(all);
    set_last_update.set(Some(
        js_sys::Date::new_0()
            .to_locale_time_string("sv-SE")
            .as_string()
            .unwrap_or_default(),
    ));
}

#[component]
pub fn SlDepartureList(departures: Vec<SlDeparture>) -> impl IntoView {
    view! {
        <div class="sl-departure-list">
            {departures
                .into_iter()
                .map(|d| view! { <SlDepartureCard departure=d/> })
                .collect_view()}
        </div>
    }
}

#[component]
fn SlDepartureCard(departure: SlDeparture) -> impl IntoView {
    let via_suffix = departure
        .via
        .as_ref()
        .map(|via| format!(" (via {via})"))
        .unwrap_or_default();
    view! {
        <div class="sl-departure">
            <div class="sl-departure-left">
                <p class="sl-stop-point-name">{format!("från: {}", departure.stop_point_name)}</p>
                <p class="sl-destination">{format!("till: {}{}", departure.destination, via_suffix)}</p>
            </div>
            <div class="sl-departure-middle">
                <SlLineBadge
                    mode=departure.transport_mode
                    line_group=departure.line_group
                    line_designation=departure.line_designation.clone()
                />
            </div>
            <div class="sl-departure-middle-right">
                <p class="sl-display-time large-text">{departure.display_time.clone()}</p>
            </div>
            <div class="sl-departure-right">
                <p class="sl-stop-point large-text">{departure.stop_point_designation.clone()}</p>
            </div>
        </div>
    }
}

fn badge_class(line_group: SlLineGroup) -> &'static str {
    match line_group {
        SlLineGroup::Train => "sl-line-badge-train",
        SlLineGroup::RedMetro => "sl-line-badge-red-metro",
        SlLineGroup::GreenMetro => "sl-line-badge-green-metro",
        SlLineGroup::BlueMetro => "sl-line-badge-blue-metro",
        SlLineGroup::Bus => "sl-line-badge-bus",
        SlLineGroup::BlueBus => "sl-line-badge-blue-bus",
        SlLineGroup::CityLine => "sl-line-badge-city-line",
        SlLineGroup::RoslagenLine => "sl-line-badge-roslagen-line",
    }
}

#[component]
fn SlLineBadge(mode: SlTransportMode, line_group: SlLineGroup, line_designation: String) -> impl IntoView {
    let class = format!("sl-line-badge {}", badge_class(line_group));
    view! {
        <div class=class>
            <div class="sl-line-number">
                <p style="font-size:1vw;padding-right:0.5vw;">{line_designation}</p>
            </div>
            <TransportIcon mode=mode/>
        </div>
    }
}

// SVGs shamelessly ported from sl.se Sök avgångar (same source the previous
// screen-frontend copied them from).
#[component]
fn TransportIcon(mode: SlTransportMode) -> impl IntoView {
    let path = match mode {
        SlTransportMode::Train | SlTransportMode::Metro | SlTransportMode::Tram => {
            "M12 19a7 7 0 100-14 7 7 0 000 14zm8-7a8 8 0 11-16 0 8 8 0 0116 0zm-6.75-1.25h3.25v-2.5h-9v2.5h3.25v6.75h2.5v-6.75z"
        }
        SlTransportMode::Bus | SlTransportMode::Taxi => {
            "M6 6a2 2 0 012-2h8a2 2 0 012 2v11a2 2 0 01-1 1.732v.018a1.25 1.25 0 01-2.475.25h-5.05A1.25 1.25 0 017 18.75v-.018A2 2 0 016 17V6zm1 2a1 1 0 011-1h8a1 1 0 011 1v5.5a2 2 0 01-2 2H9a2 2 0 01-2-2V8zm2.5-3a.5.5 0 000 1h5a.5.5 0 000-1h-5zM15 17a.5.5 0 01.5-.5h1a.5.5 0 01.5.5v.5a.5.5 0 01-.5.5h-1a.5.5 0 01-.5-.5V17zm-7.5-.5a.5.5 0 00-.5.5v.5a.5.5 0 00.5.5h1a.5.5 0 00.5-.5V17a.5.5 0 00-.5-.5h-1z"
        }
        SlTransportMode::Ferry | SlTransportMode::Ship => {
            "M11 5v-.777c0-.075.078-.14.19-.158a5.083 5.083 0 011.62 0c.112.018.19.083.19.158V5h-2zM6.504 15L6.5 16c.45 0 .697-.101.97-.212.334-.136.706-.288 1.531-.288.826 0 1.348.303 1.818.576.383.222.732.424 1.181.424.45 0 .798-.202 1.183-.424.47-.273.994-.576 1.819-.576s1.196.152 1.53.288c.272.111.519.212.968.212v-1c.002 0 .882-1.993 1.295-2.927a.499.499 0 00-.273-.665L17.5 11V7.5a2 2 0 00-2-2H8.502a2 2 0 00-2 1.999L6.5 11l-1.019.408a.5.5 0 00-.271.666L6.504 15zm1.43 4.368c-.539.308-1.21.632-2.434.632H4v-2H5.5c.776 0 1.105-.176 1.441-.368l.095-.055C7.459 17.332 8.032 17 9 17c1.05 0 1.63.385 2.055.668l.004.003c.323.215.494.329.941.329.456 0 .647-.12.98-.338l.02-.013c.414-.27.994-.649 2-.649.928 0 1.503.329 1.916.565l.11.063c.346.194.694.372 1.474.372H20v2H18.5c-1.22 0-1.906-.322-2.452-.628l-.1-.056c-.396-.223-.56-.316-.948-.316-.41 0-.577.109-.928.338-.435.283-1.028.662-2.072.662-1.05 0-1.63-.385-2.055-.668l-.004-.003C9.618 19.114 9.447 19 9 19c-.425 0-.598.1-.98.319l-.086.05zM16 7H8v3.5L12 9l4 1.5V7z"
        }
    };
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="2vw" height="2vw" fill="none" viewBox="0 0 24 24">
            <rect width="24" height="24" fill="var(--color-dark-gray)" rx="4"></rect>
            <path fill="var(--color-white)" fill-rule="evenodd" d=path clip-rule="evenodd"></path>
        </svg>
    }
}
