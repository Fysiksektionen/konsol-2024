//! Small client-only helper so components don't need to sprinkle
//! `#[cfg(feature = "hydrate")]` themselves; the real implementation only
//! ever runs from inside `Effect::new`, which Leptos never executes during
//! SSR, so the no-op stub only matters for making the *ssr* target compile.

#[cfg(feature = "hydrate")]
pub fn set_interval(f: impl FnMut() + 'static, millis: i32) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let closure = Closure::<dyn FnMut()>::new(f).into_js_value();
    web_sys::window()
        .expect("no window")
        .set_interval_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), millis)
        .expect("failed to set interval");
}

#[cfg(not(feature = "hydrate"))]
pub fn set_interval(_f: impl FnMut() + 'static, _millis: i32) {}

/// `web_sys::window()` returns `None` server-side (no global JS `window`),
/// so this is safe to call unconditionally; it just always denies on SSR.
pub fn window_confirm(message: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.confirm_with_message(message).ok())
        .unwrap_or(false)
}

pub fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        format!("{}...", s.chars().take(max).collect::<String>())
    } else {
        s.to_string()
    }
}

