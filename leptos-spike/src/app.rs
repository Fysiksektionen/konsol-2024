use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

// ---------------------------------------------------------------------------
// Spike 1: multipart file upload through a server function.
//
// Mirrors backend/src/routes.rs's `save_slide` handler, which today takes an
// actix-multipart form. Here the "API" is just an async Rust function; there
// is no hand-written route, request struct, or JSON contract.
// ---------------------------------------------------------------------------

#[server(input = server_fn::codec::MultipartFormData)]
pub async fn upload_slide(data: server_fn::codec::MultipartData) -> Result<String, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let mut received = Vec::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.file_name().unwrap_or_default().to_string();
        let mut total_bytes = 0usize;
        while let Ok(Some(chunk)) = field.chunk().await {
            total_bytes += chunk.len();
        }
        received.push(format!("{name} ({total_bytes} bytes)"));
    }

    Ok(format!("received: {}", received.join(", ")))
}

#[component]
pub fn FileUploadSpike() -> impl IntoView {
    let (result, set_result) = signal(String::new());

    let upload = Action::new(move |data: &web_sys::FormData| {
        let data = data.clone();
        async move {
            match upload_slide(data.into()).await {
                Ok(msg) => set_result.set(msg),
                Err(e) => set_result.set(format!("error: {e}")),
            }
        }
    });

    view! {
        <form on:submit=move |ev: leptos::ev::SubmitEvent| {
            use wasm_bindgen::JsCast;
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<web_sys::HtmlFormElement>();
            let form_data = web_sys::FormData::new_with_form(&target).unwrap();
            upload.dispatch(form_data);
        }>
            <input type="file" name="file_to_upload"/>
            <input type="submit" value="Upload"/>
        </form>
        <p>{move || result.get()}</p>
    }
}

// ---------------------------------------------------------------------------
// Spike 2: Google OAuth JS interop bridged to a server function.
//
// Google's Identity Services SDK is plain JS (same category of thing Blazor
// still drops into JS interop for). Its callback dispatches a CustomEvent,
// which a Leptos effect listens for and forwards into a server function —
// same shape as today's POST /api/auth/verify, minus the hand-written route.
//
// NOTE: there is no registered Google OAuth client in this sandbox, so the
// real `g_id_signin` button can't be clicked end-to-end here. The "Simulate"
// button fires the identical CustomEvent a real Google callback would, to
// prove the JS -> WASM -> server-function bridge itself works.
// ---------------------------------------------------------------------------

#[server]
pub async fn verify_google_credential(credential: String) -> Result<String, ServerFnError> {
    if credential.is_empty() {
        return Err(ServerFnError::new("empty credential"));
    }
    // Stand-in for today's google-oauth crate verification + users-table
    // lookup + session cookie set (routes.rs's /api/auth/verify).
    Ok(format!("verified credential of length {}", credential.len()))
}

#[component]
pub fn GoogleLoginSpike() -> impl IntoView {
    let (status, set_status) = signal(String::from("waiting for credential..."));

    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        let closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::Event)>::new(
            move |ev: web_sys::Event| {
                let custom: web_sys::CustomEvent = ev.unchecked_into();
                if let Some(credential) = custom.detail().as_string() {
                    set_status.set("verifying...".to_string());
                    leptos::task::spawn_local(async move {
                        match verify_google_credential(credential).await {
                            Ok(msg) => set_status.set(msg),
                            Err(e) => set_status.set(format!("error: {e}")),
                        }
                    });
                }
            },
        );
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback(
                "google-credential",
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        closure.forget();
    });

    view! {
        <div>
            <div
                id="g_id_onload"
                data-client_id="SPIKE-CLIENT-ID.apps.googleusercontent.com"
                data-callback="handleCredentialResponse"
            ></div>
            <div class="g_id_signin"></div>
            <p>{move || status.get()}</p>
            <button on:click=move |_| {
                use wasm_bindgen::JsValue;
                let init = web_sys::CustomEventInit::new();
                init.set_detail(&JsValue::from_str("fake.jwt.credential"));
                let event = web_sys::CustomEvent::new_with_event_init_dict(
                    "google-credential",
                    &init,
                )
                .unwrap();
                web_sys::window().unwrap().dispatch_event(&event).unwrap();
            }>
                "Simulate Google credential (dev only, no real client id here)"
            </button>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Shell / routing
// ---------------------------------------------------------------------------

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
                <script src="https://accounts.google.com/gsi/client" async=true defer=true></script>
                <script>
                    r#"function handleCredentialResponse(response) {
                        window.dispatchEvent(new CustomEvent('google-credential', { detail: response.credential }));
                    }"#
                </script>
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
        <Stylesheet id="leptos" href="/pkg/leptos-spike.css"/>
        <Title text="KONSol Leptos spike"/>
        <Router>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"KONSol -> Leptos spike"</h1>
        <section>
            <h2>"1. Multipart file upload via server function"</h2>
            <FileUploadSpike/>
        </section>
        <section>
            <h2>"2. Google OAuth JS interop via server function"</h2>
            <GoogleLoginSpike/>
        </section>
    }
}
