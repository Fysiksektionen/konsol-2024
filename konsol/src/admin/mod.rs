use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_router::components::{Outlet, A};
use wasm_bindgen::JsCast;

use crate::api::auth::{
    add_user, auth_status, get_google_client_id, list_users, logout, remove_user, verify_google_credential,
};
use crate::api::slides::{delete_slide, get_slides, upload_slide};
use crate::client_util::{truncate_chars, window_confirm};
use crate::models::{AuthenticatedUser, PermissionLevel, Slide, User};

/// `None` = auth not checked yet, `Some(None)` = checked but not logged in,
/// `Some(Some(user))` = logged in. Mirrors the previous admin-frontend's
/// `User | null | undefined` state exactly.
type AuthState = Option<Option<AuthenticatedUser>>;

#[component]
pub fn AdminApp() -> impl IntoView {
    let (user, set_user) = signal::<AuthState>(None);
    provide_context(user);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match auth_status().await {
                Ok(u) => set_user.set(Some(Some(u))),
                Err(_) => set_user.set(Some(None)),
            }
        });
    });

    view! {
        <Stylesheet id="admin" href="/admin.css"/>
        <script src="https://accounts.google.com/gsi/client" async=true defer=true></script>
        <script>
            r#"function handleCredentialResponse(response) {
                window.dispatchEvent(new CustomEvent('google-credential', { detail: response.credential }));
            }"#
        </script>
        <div class="app-container">
            <Header user=user set_user=set_user/>
            <NavHeader user=user/>
            {move || match user.get() {
                None => view! { <p style="text-align:center;">"Loading..."</p> }.into_any(),
                Some(None) => view! { <p style="text-align:center;">"You are not logged in."</p> }.into_any(),
                Some(Some(_)) => view! { <Outlet/> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn Header(user: ReadSignal<AuthState>, set_user: WriteSignal<AuthState>) -> impl IntoView {
    view! {
        <div class="header">
            <div class="header-left">
                <img src="https://f.kth.se/wp-content/uploads/FysikMedium.png" class="logo"/>
                <h1 class="title">"Konsol Admin"</h1>
            </div>
            <div class="header-right">
                <div class="user-status">
                    <UserStatus user=user set_user=set_user/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NavHeader(user: ReadSignal<AuthState>) -> impl IntoView {
    let is_admin = move || matches!(user.get(), Some(Some(u)) if u.permission == PermissionLevel::Admin);
    view! {
        <div class="nav-header">
            <A href="/konsol/admin/slides" attr:class="slides-page-button">
                "Slides"
            </A>
            <Show when=is_admin>
                <A href="/konsol/admin/users" attr:class="users-page-button">
                    "Users"
                </A>
            </Show>
        </div>
    }
}

#[component]
fn UserStatus(user: ReadSignal<AuthState>, set_user: WriteSignal<AuthState>) -> impl IntoView {
    // LocalResource (not Resource): this is only ever needed client-side to
    // render the Google button, and reading a Resource outside a Suspense
    // caused a hydration-mismatch warning that made the button's DOM node
    // get torn down and recreated right after Google's script had already
    // wired an interactive button into it — LocalResource never resolves
    // during SSR, so there's nothing for hydration to mismatch against.
    let client_id = LocalResource::new(|| get_google_client_id());

    Effect::new(move |_| {
        let closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::Event)>::new(
            move |ev: web_sys::Event| {
                let custom: web_sys::CustomEvent = ev.unchecked_into();
                let Some(credential) = custom.detail().as_string() else {
                    return;
                };
                leptos::task::spawn_local(async move {
                    match verify_google_credential(credential).await {
                        Ok(u) => set_user.set(Some(Some(u))),
                        Err(e) => leptos::logging::error!("Login failed: {e}"),
                    }
                });
            },
        );
        if let Some(window) = web_sys::window() {
            let _ =
                window.add_event_listener_with_callback("google-credential", closure.as_ref().unchecked_ref());
        }
        closure.forget();
    });

    let handle_logout = move |_| {
        leptos::task::spawn_local(async move {
            let _ = logout().await;
            set_user.set(Some(None));
        });
    };

    view! {
        {move || match user.get() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(None) => {
                match client_id.get() {
                    Some(Ok(id)) => {
                        view! {
                            <div>
                                <div id="g_id_onload" data-client_id=id data-callback="handleCredentialResponse"></div>
                                <div class="g_id_signin"></div>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        leptos::logging::error!("Failed to fetch Google client ID: {e}");
                        view! { <p>"Login unavailable"</p> }.into_any()
                    }
                    None => view! { <p>"Loading..."</p> }.into_any(),
                }
            }
            Some(Some(u)) => {
                let is_admin = u.permission == PermissionLevel::Admin;
                view! {
                    <>
                        <p on:click=handle_logout style="cursor:pointer;">
                            {u.email.clone()}
                        </p>
                        <Show when=move || is_admin>
                            <p>"Admin"</p>
                        </Show>
                    </>
                }
                    .into_any()
            }
        }}
    }
}

#[component]
pub fn SlidesPage() -> impl IntoView {
    let slides = Resource::new(|| (), |_| get_slides());
    let (show_popup, set_show_popup) = signal(false);

    let upload = Action::new(move |data: &web_sys::FormData| {
        let data = data.clone();
        async move {
            let result = upload_slide(data.into()).await;
            if result.is_ok() {
                slides.refetch();
                set_show_popup.set(false);
            }
            result
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<web_sys::HtmlFormElement>();
        let form_data = web_sys::FormData::new_with_form(&target).unwrap();
        upload.dispatch(form_data);
    };

    view! {
        <div class="slides-page">
            <div class="slides-header">
                <h1>"Slides"</h1>
                <button class="add-slide-button" on:click=move |_| set_show_popup.set(true)>
                    "Add Slide"
                </button>
            </div>
            <Show when=move || show_popup.get()>
                <div
                    class="add-slide-popup-overlay"
                    style="position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:2000;"
                >
                    <div class="add-slide-popup-content" style="padding:2rem;min-width:20rem;">
                        <h2>"Add Slide"</h2>
                        <form on:submit=on_submit>
                            <label for="caption">"Caption"</label>
                            <input type="text" id="caption" name="caption"/>
                            <label for="startDate">"Start Date"</label>
                            <input type="date" id="startDate" name="start"/>
                            <label for="endDate">"End Date"</label>
                            <input type="date" id="endDate" name="end"/>
                            <label for="file">"Image"</label>
                            <input type="file" id="file" name="imageFile"/>
                            <button type="submit">"Submit"</button>
                        </form>
                        <button on:click=move |_| set_show_popup.set(false)>"Close"</button>
                    </div>
                </div>
            </Show>
            <div class="slides">
                <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                    {move || {
                        slides
                            .get()
                            .and_then(|r| r.ok())
                            .unwrap_or_default()
                            .into_iter()
                            .map(|slide| {
                                view! { <SlideCard slide=slide on_removed=move || slides.refetch()/> }
                            })
                            .collect_view()
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn SlideCard(slide: Slide, on_removed: impl Fn() + Copy + 'static) -> impl IntoView {
    let id_for_remove = slide.id.clone();
    let handle_remove = move |_| {
        let id = id_for_remove.clone();
        leptos::task::spawn_local(async move {
            if delete_slide(id).await.is_ok() {
                on_removed();
            }
        });
    };

    let truncated = truncate_chars(&slide.caption, 30);
    let indicator_class = format!("indicator {}", if slide.active { "active" } else { "inactive" });
    let src = format!("/screen/slides/images/{}.{}", slide.id, slide.filetype);
    let date_range = format!(
        "{} \u{2013} {}",
        slide.start_date.format("%Y-%m-%d"),
        slide.end_date.format("%Y-%m-%d")
    );

    view! {
        <div class="slide">
            <div class=indicator_class></div>
            <h2>{truncated}</h2>
            <img class="slide-image" src=src alt=slide.caption.clone()/>
            <p>{date_range}</p>
            <button class="remove-button" on:click=handle_remove>
                "X"
            </button>
        </div>
    }
}

#[component]
pub fn UsersPageGate() -> impl IntoView {
    let user = use_context::<ReadSignal<AuthState>>().expect("user context provided by AdminApp");
    view! {
        {move || match user.get() {
            Some(Some(u)) if u.permission == PermissionLevel::Admin => view! { <UsersPage/> }.into_any(),
            _ => {
                view! {
                    <div class="access-denied-page">
                        <p style="text-align:center;">"You do not have permission to view this page."</p>
                    </div>
                }
                    .into_any()
            }
        }}
    }
}

#[component]
fn UsersPage() -> impl IntoView {
    let users = Resource::new(|| (), |_| list_users());
    let (show_popup, set_show_popup) = signal(false);

    let add = Action::new(move |(email, admin): &(String, bool)| {
        let email = email.clone();
        let permission = if *admin { PermissionLevel::Admin } else { PermissionLevel::User };
        async move {
            let result = add_user(email, permission).await;
            if result.is_ok() {
                users.refetch();
                set_show_popup.set(false);
            }
            result
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<web_sys::HtmlFormElement>();
        let form_data = web_sys::FormData::new_with_form(&target).unwrap();
        let email = form_data.get("email").as_string().unwrap_or_default();
        let admin = form_data.get("admin").as_string().as_deref() == Some("on");
        add.dispatch((email, admin));
    };

    view! {
        <div class="users-page">
            <div class="users-header">
                <h1>"Users"</h1>
                <button class="add-user-button" on:click=move |_| set_show_popup.set(true)>
                    "Add User"
                </button>
            </div>
            <Show when=move || show_popup.get()>
                <div
                    class="add-user-popup-overlay"
                    style="position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:2000;"
                >
                    <div class="add-user-popup-content" style="padding:2rem;min-width:20rem;">
                        <h2>"Add User"</h2>
                        <form on:submit=on_submit>
                            <label>"Email:"</label>
                            <input type="email" name="email" required=true/>
                            <label>"Admin:"</label>
                            <input type="checkbox" name="admin"/>
                            <button type="submit">"Submit"</button>
                        </form>
                        <button on:click=move |_| set_show_popup.set(false)>"Close"</button>
                    </div>
                </div>
            </Show>
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || {
                    users
                        .get()
                        .and_then(|r| r.ok())
                        .unwrap_or_default()
                        .into_iter()
                        .map(|u| view! { <UserCard user_data=u on_removed=move || users.refetch()/> })
                        .collect_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn UserCard(user_data: User, on_removed: impl Fn() + Copy + 'static) -> impl IntoView {
    let id_for_remove = user_data.id.clone();
    let email_for_confirm = user_data.email.clone();
    let handle_remove = move |_| {
        if !window_confirm(&format!("Are you sure you want to remove user {email_for_confirm}?")) {
            return;
        }
        let id = id_for_remove.clone();
        leptos::task::spawn_local(async move {
            if remove_user(id).await.is_ok() {
                on_removed();
            }
        });
    };

    view! {
        <div class="user">
            <p>{user_data.email.clone()}</p>
            <p>{if user_data.admin { "Admin" } else { "Not admin" }}</p>
            <button class="remove-user-button" on:click=handle_remove>
                "Remove User"
            </button>
        </div>
    }
}
