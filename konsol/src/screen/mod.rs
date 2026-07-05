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
                let layout = settings
                    .get()
                    .and_then(|r| r.ok())
                    .map(|s| s.layout_type)
                    .unwrap_or_else(|| "mixed".to_string());
                let slides_list = slides.get().and_then(|r| r.ok()).unwrap_or_default();
                match layout.as_str() {
                    "fullscreen_slideshow" => {
                        view! { <FullscreenSlideshowLayout slides=slides_list/> }.into_any()
                    }
                    _ => {
                        view! {
                            <MixedLayout
                                slides=slides_list
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
fn FullscreenSlideshowLayout(slides: Vec<Slide>) -> impl IntoView {
    view! { <Slideshow slides=slides/> }
}

#[component]
fn MixedLayout(slides: Vec<Slide>, departures: Vec<SlDeparture>, last_update: Option<String>) -> impl IntoView {
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

    let has_slides = !slides.is_empty();
    let last_update_text = last_update
        .map(|t| format!("Senast uppdaterad: {t}"))
        .unwrap_or_else(|| "Senast uppdaterad: Aldrig".to_string());

    view! {
        <div class="header">
            <img src="/assets/FrakturF2020.png" alt="Fraktur F" class="fysikf"/>
            <h1>"KONSol"</h1>
        </div>

        <div class="left">
            {move || {
                if active.get() == "slide" && has_slides {
                    view! { <Slideshow slides=slides.clone()/> }.into_any()
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
