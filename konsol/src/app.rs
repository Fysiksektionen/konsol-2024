use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::admin::{AdminApp, SettingsPage, SlidesPage, UsersPageGate};
use crate::screen::ScreenApp;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <Route path=(StaticSegment("konsol"), StaticSegment("screen")) view=ScreenApp/>
                <ParentRoute path=(StaticSegment("konsol"), StaticSegment("admin")) view=AdminApp>
                    <Route path=StaticSegment("") view=SlidesPage/>
                    <Route path=StaticSegment("slides") view=SlidesPage/>
                    <Route path=StaticSegment("settings") view=SettingsPage/>
                    <Route path=StaticSegment("users") view=UsersPageGate/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
