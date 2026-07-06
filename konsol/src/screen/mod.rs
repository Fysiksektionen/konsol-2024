mod calendar;
pub mod sl;

use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::api::settings::get_settings;
use crate::api::slides::get_slides;
use crate::client_util::set_interval;
use crate::models::Slide;
use calendar::Calendar;
use sl::{SlDeparture, SlDepartureList};

#[component]
pub fn ScreenApp() -> impl IntoView {
    let slides = Resource::new(|| (), |_| get_slides());
    let settings = Resource::new(|| (), |_| get_settings());

    let (departures, set_departures) = signal(Vec::<SlDeparture>::new());
    let (last_update, set_last_update) = signal(None::<String>);

    Effect::new(move |_| {
        sl::start_polling(set_departures, set_last_update);
    });

    view! {
        <Stylesheet id="screen" href="/screen.css"/>
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || {
                let s = settings.get().and_then(|r| r.ok()).unwrap_or_default();
                let live_url =
                    (s.live_mode && !s.live_slides_url.is_empty()).then(|| s.live_slides_url.clone());
                let slides_list = slides.get().and_then(|r| r.ok()).unwrap_or_default();
                match s.layout_type.as_str() {
                    "fullscreen_slideshow" => {
                        view! { <FullscreenSlideshowLayout slides=slides_list live_url=live_url/> }
                            .into_any()
                    }
                    _ => {
                        view! {
                            <MixedLayout
                                slides=slides_list
                                live_url=live_url
                                departures=departures.get()
                                last_update=last_update.get()
                            />
                        }
                            .into_any()
                    }
                }
            }}
        </Suspense>
    }
}

#[component]
fn FullscreenSlideshowLayout(slides: Vec<Slide>, live_url: Option<String>) -> impl IntoView {
    view! {
        <div id="root">
            {match live_url {
                Some(url) => view! { <LiveSlides url=url/> }.into_any(),
                None => view! { <Slideshow slides=slides/> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn MixedLayout(
    slides: Vec<Slide>,
    live_url: Option<String>,
    departures: Vec<SlDeparture>,
    last_update: Option<String>,
) -> impl IntoView {
    let (active, set_active) = signal("slide".to_string());

    Effect::new(move |_| {
        set_interval(
            move || {
                set_active
                    .update(|a| *a = if a == "slide" { "calendar".to_string() } else { "slide".to_string() });
            },
            15_000,
        );
    });

    let has_slides = !slides.is_empty() || live_url.is_some();
    let last_update_text = last_update
        .map(|t| format!("Senast uppdaterad: {t}"))
        .unwrap_or_else(|| "Senast uppdaterad: Aldrig".to_string());

    view! {
        <div id="root">
            <div class="header">
                <img src="/assets/FrakturF2020.png" alt="Fraktur F" class="fysikf"/>
                <h1>"KONSol"</h1>
            </div>

            <div class="left">
                {move || {
                    if active.get() == "slide" && has_slides {
                        if let Some(url) = live_url.clone() {
                            view! { <LiveSlides url=url/> }.into_any()
                        } else {
                            view! { <Slideshow slides=slides.clone()/> }.into_any()
                        }
                    } else {
                        view! {
                            <div class="calendar-container">
                                <Calendar/>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>

            <div class="right">
                <p class="last-update">{last_update_text}</p>
                <SlDepartureList departures=departures/>
            </div>
        </div>
    }
}

#[component]
fn Slideshow(slides: Vec<Slide>) -> impl IntoView {
    let (index, set_index) = signal(0usize);
    let len = slides.len();

    if len == 0 {
        return view! { <div>"Loading slides..."</div> }.into_any();
    }

    Effect::new(move |_| {
        set_interval(
            move || {
                set_index.update(|i| *i = (*i + 1) % len);
            },
            3_000,
        );
    });

    view! {
        <div>
            {move || {
                let slide = slides[index.get()].clone();
                let src = format!("/screen/slides/images/{}.{}", slide.id, slide.filetype);
                view! {
                    <div>
                        <h2>{slide.caption.clone()}</h2>
                        <img class="slide-image" src=src alt=slide.caption/>
                    </div>
                }
            }}
            <button on:click=move |_| set_index.update(|i| *i = (*i + len - 1) % len)>"Previous"</button>
            <button on:click=move |_| set_index.update(|i| *i = (*i + 1) % len)>"Next"</button>
        </div>
    }
        .into_any()
}

/// How often the live embed reloads to pick up edits. Google republishes an
/// edited presentation on its own after a short delay, so this only bounds
/// how stale the screen can get; each reload also restarts the deck from its
/// first slide, which is why this isn't lower.
const LIVE_RELOAD_INTERVAL_MS: i32 = 3 * 60_000;

/// Turn whatever Google Slides link was pasted into settings (an /edit link
/// from the address bar, a published /pub link, or already an /embed link)
/// into the embedded-player URL with auto-advance and looping enabled.
fn google_slides_embed_url(url: &str) -> String {
    let base = url.split(['#', '?']).next().unwrap_or(url);
    let base = base.trim_end_matches('/');
    let base = base
        .strip_suffix("/edit")
        .or_else(|| base.strip_suffix("/pub"))
        .or_else(|| base.strip_suffix("/embed"))
        .or_else(|| base.strip_suffix("/present"))
        .or_else(|| base.strip_suffix("/preview"))
        .unwrap_or(base);
    format!("{base}/embed?start=true&loop=true&delayms=3000")
}

#[component]
fn LiveSlides(url: String) -> impl IntoView {
    let embed = google_slides_embed_url(&url);
    let (generation, set_generation) = signal(0u32);

    Effect::new(move |_| {
        set_interval(move || set_generation.update(|g| *g += 1), LIVE_RELOAD_INTERVAL_MS);
    });

    view! {
        <div class="live-slides">
            {move || {
                // The changing cache-buster param forces the iframe to reload,
                // which is what actually pulls in edits to the presentation.
                let src = format!("{embed}&cb={}", generation.get());
                view! { <iframe src=src allowfullscreen=true></iframe> }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::google_slides_embed_url;

    const EMBED: &str = "https://docs.google.com/presentation/d/abc123/embed?start=true&loop=true&delayms=3000";

    #[test]
    fn edit_link() {
        assert_eq!(
            google_slides_embed_url("https://docs.google.com/presentation/d/abc123/edit#slide=id.p"),
            EMBED
        );
    }

    #[test]
    fn published_link() {
        assert_eq!(
            google_slides_embed_url(
                "https://docs.google.com/presentation/d/abc123/pub?start=false&loop=false&delayms=60000"
            ),
            EMBED
        );
    }

    #[test]
    fn embed_link() {
        assert_eq!(google_slides_embed_url(EMBED), EMBED);
    }

    #[test]
    fn bare_document_link() {
        assert_eq!(google_slides_embed_url("https://docs.google.com/presentation/d/abc123/"), EMBED);
    }
}
