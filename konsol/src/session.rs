#![cfg(feature = "ssr")]

//! Session handling: the authenticated user is stored directly in an
//! encrypted+signed cookie (mirroring the previous actix-session
//! `CookieSessionStore` behavior) rather than a server-side session store,
//! so a server restart doesn't log everyone out as long as
//! `COOKIE_SECRET_KEY` stays the same.

use axum::http::header::{COOKIE, SET_COOKIE};
use axum::http::HeaderMap;
use cookie::{Cookie, CookieJar, Key, SameSite};
use leptos::prelude::*;

use crate::models::{AuthenticatedUser, PermissionLevel};

const COOKIE_NAME: &str = "auth";

fn cookie_key() -> Key {
    let secret = std::env::var("COOKIE_SECRET_KEY").unwrap_or_else(|_| {
        log::warn!("COOKIE_SECRET_KEY not set; using default insecure key");
        "hejjakwdjaklwjdlka jwkldjalkwdjalkwjdlkajlkdja klwd231ahwdah widuhalwiudh aliuwhdjlad"
            .to_string()
    });
    Key::from(secret.as_bytes())
}

fn cookie_secure() -> bool {
    std::env::var("COOKIE_SECURE")
        .unwrap_or_else(|_| {
            log::warn!("COOKIE_SECURE not set; defaulting to false");
            "false".to_string()
        })
        .eq_ignore_ascii_case("true")
}

async fn incoming_jar() -> Result<CookieJar, ServerFnError> {
    let headers: HeaderMap = leptos_axum::extract().await?;
    let mut jar = CookieJar::new();
    if let Some(cookie_header) = headers.get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for pair in Cookie::split_parse_encoded(cookie_str.to_string()).flatten() {
                jar.add_original(pair.into_owned());
            }
        }
    }
    Ok(jar)
}

fn write_set_cookie_headers(jar: &CookieJar) {
    let response_opts = expect_context::<leptos_axum::ResponseOptions>();
    for cookie in jar.delta() {
        if let Ok(value) = cookie.encoded().to_string().parse() {
            response_opts.insert_header(SET_COOKIE, value);
        }
    }
}

pub async fn get_authenticated_user() -> Result<Option<AuthenticatedUser>, ServerFnError> {
    let jar = incoming_jar().await?;
    let key = cookie_key();
    let Some(private_cookie) = jar.private(&key).get(COOKIE_NAME) else {
        return Ok(None);
    };
    Ok(serde_json::from_str(private_cookie.value()).ok())
}

pub async fn set_authenticated_user(user: &AuthenticatedUser) -> Result<(), ServerFnError> {
    let json = serde_json::to_string(user).map_err(|e| ServerFnError::new(e.to_string()))?;
    let mut jar = CookieJar::new();
    let key = cookie_key();
    let cookie = Cookie::build((COOKIE_NAME, json))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(cookie_secure())
        .build();
    jar.private_mut(&key).add(cookie);
    write_set_cookie_headers(&jar);
    Ok(())
}

pub fn clear_authenticated_user() {
    let mut jar = CookieJar::new();
    jar.add(Cookie::new(COOKIE_NAME, ""));
    jar.remove(COOKIE_NAME);
    write_set_cookie_headers(&jar);
}

pub async fn require_auth() -> Result<AuthenticatedUser, ServerFnError> {
    get_authenticated_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Not logged in"))
}

pub async fn require_admin() -> Result<AuthenticatedUser, ServerFnError> {
    let user = require_auth().await?;
    if user.permission == PermissionLevel::Admin {
        Ok(user)
    } else {
        Err(ServerFnError::new("Forbidden"))
    }
}
