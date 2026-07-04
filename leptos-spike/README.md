# Leptos spike (not part of the KONSol app)

A throwaway prototype, not wired into the real KONSol backend/frontends. It
exists to de-risk two specific unknowns before committing to rewriting
`admin-frontend`/`screen-frontend` in Leptos + Axum (replacing the current
Rust/Actix + React/TS stack — see root `CLAUDE.md`):

1. **Multipart file upload through a Leptos server function** — stand-in for
   `backend/src/routes.rs`'s `save_slide` handler (currently `actix-multipart`).
2. **Google OAuth JS interop bridged into a server function** — stand-in for
   `POST /api/auth/verify` — proving a plain-JS SDK (unavoidable for Google's
   login button, same as Blazor apps still drop into JS interop for this) can
   hand a credential to Rust/WASM, which calls a server function exactly like
   calling a local async function — no hand-written REST route or JSON DTO.

## Result: both work

Verified with `cargo build` (server + client-side `cargo check` both compile
clean) **and** end-to-end in a real headless-Chromium session
(`playwright`, driving the actual hydrated WASM app, not just curl):

- Uploading a file through the real `<input type="file">` form returns
  `"received: <filename> (<n> bytes)"` from the server.
- Clicking "Simulate Google credential" dispatches a `CustomEvent`, which a
  `wasm_bindgen::closure::Closure` event listener picks up, calls
  `verify_google_credential(...)` as a server function, and the result flows
  back into a reactive signal and re-renders in the DOM.

There's no real Google OAuth client registered in this environment, so the
actual `g_id_signin` button can't be clicked for real — the "Simulate" button
fires the identical `CustomEvent` a real Google callback would produce, to
isolate and prove the JS → WASM → server-function bridge on its own.

## Things that were harder than expected (worth knowing before a real rewrite)

- **nixpkgs lags the leptos release cadence badly.** The repo's `flake.nix`
  originally seemed like the right place to add `cargo-leptos` +
  `wasm-bindgen-cli`, but nixpkgs' `cargo-leptos` (0.2.42) statically bundles
  a `wasm-bindgen` version (0.2.100) two years behind what current `leptos`
  (0.8.20) requires (0.2.126) — and nixpkgs' own pinned `rustc` (1.88) is too
  old to even build a newer `cargo-leptos` from source. Ended up using
  `cargo install cargo-leptos` + `cargo install wasm-bindgen-cli --version
  <exact match>` via the ambient rustup toolchain instead. If this becomes a
  real dependency, the project's nix flake needs a rust-overlay/fenix-style
  setup with explicit version control, not plain nixpkgs `rustc`/`cargo`.
- **The `leptos`/`leptos_router`/`leptos_meta`/`leptos_axum` family crates
  version independently** (not in lockstep numbers) but must resolve to a
  mutually-compatible set — a loose `"0.8"` requirement can let Cargo pick a
  newer `leptos_macro` than the `leptos` lib crate it's paired with, breaking
  the build with `cannot find function` errors from macro-generated code.
  Exact-pinning just `leptos` itself (not the others) fixed it here.
- **Multipart requires an explicit `multipart` feature flag** on `leptos`
  (which forwards to `server_fn/multipart`) — not on by default.
- Server function endpoint paths aren't just `/api/<fn_name>`; a content hash
  gets appended (e.g. `/api/upload_slide14204214554510480350`). Not a problem
  for calls made through the generated client stubs, but worth knowing if
  anything ever needs to hit these routes directly (e.g. manual curl testing,
  as done here).

## Running it

```
cargo leptos build   # or: cargo leptos watch
./target/debug/leptos-spike   # binds 127.0.0.1:3000 by default
```

Needs `cargo-leptos` and a `wasm-bindgen-cli` whose version exactly matches
the `wasm-bindgen` crate version in `Cargo.lock` (see note above).
