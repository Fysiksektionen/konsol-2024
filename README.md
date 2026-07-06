# konsol-2024 (WIP)
KONSol is an information screen in Konsulatet. The app lives in `konsol/`: a single
full-stack Rust application built with [Leptos](https://leptos.dev/) and Axum — the
server renders and serves both the screen and the admin page, and the browser side
is the same Rust code compiled to WebAssembly. It replaces the previous three-part
setup (Actix backend + two React frontends), which is still in the repo under
`backend/`, `admin-frontend/` and `screen-frontend/` until the migration is done.

## Features:
- Ability to upload slides (images, title, text, date) that will be shown on the screen.
- Live mode: point the screen at a public Google Slides presentation and it follows along as the presentation is edited.
- SL timetable data.
- Fysiksektionen calendar integration.

## Configuration
Environment variables (in development, put them in `konsol/.env`):
- `DATABASE_URL`: SQLite database, e.g. `sqlite://./konsol.db`.
- `GOOGLE_ID_TOKEN`: Google OAuth client ID, used both to render the sign-in button and to verify credentials.
- `GOOGLE_CALENDAR_API_KEY`: API key for the public Google Calendar API. If unset, the screen shows mock calendar events.
- `COOKIE_SECRET_KEY`: Secret for the encrypted session cookie (falls back to an insecure dev key).
- `COOKIE_SECURE`: `true` in production (cookie only sent over HTTPS).

## Development setup
1. Make sure Rust is installed and `cargo` works. Otherwise, [install Rust](https://www.rust-lang.org/tools/install).
2. Install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos` (or `cargo binstall cargo-leptos`), and the WASM target: `rustup target add wasm32-unknown-unknown`.
3. Navigate to `konsol/` and run `cargo leptos watch` (rebuilds on change) or `cargo leptos serve`. Database migrations run automatically at startup.
4. Open `http://127.0.0.1:8080/konsol/screen` or `/konsol/admin`.

To log in to the admin page your Google account's email must exist in the `users`
table; insert the first one manually, e.g.
`sqlite3 konsol.db "INSERT INTO users (id, email, admin) VALUES ('some-id', 'you@example.com', TRUE);"`.

## Deployment
Everything the app serves — pages, server functions, JS/WASM, stylesheets, slide
images — lives under the `/konsol/` path prefix, so the deployment host's reverse
proxy only needs to route `/konsol/` to the container.

- `konsol/Dockerfile` builds the app with cargo-leptos and produces a self-contained
  image running the server on port 8080. Persistent state is the SQLite database
  (`/app/data`) and the uploaded slide images (`/app/slide_images`).
- `docker-compose.yml` runs the app together with an nginx reverse proxy
  (`outer-nginx.conf`) that mirrors the routing the production host does.
- Pushing a `v*` tag builds and publishes the image to GHCR
  (`.github/workflows/docker-konsol.yml`).

### Nix
If you don't use Nix, you can ignore this and all `*.nix`-files. If you use Nix, this project has a dev shell which can be entered with `nix develop` (if you use flakes) or `nix-shell` (if you don't).
