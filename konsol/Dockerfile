# Build the full-stack Leptos app: cargo-leptos compiles the server binary
# (ssr feature) and the browser WASM bundle (hydrate feature), and collects
# the static assets from public/ into target/site.
FROM rust:1.90 AS builder

RUN rustup target add wasm32-unknown-unknown
# Prebuilt cargo-leptos via cargo-binstall (compiling it from source adds
# ~10 minutes to the image build). cargo-leptos downloads matching
# wasm-bindgen/wasm-opt binaries itself during the build.
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
        | bash \
    && cargo binstall cargo-leptos -y

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY public ./public
COPY src ./src

RUN cargo leptos build --release

FROM debian:bookworm-slim

WORKDIR /app

# libsqlite3: diesel links it dynamically; libssl3 + ca-certificates: Google
# token verification over HTTPS.
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/konsol ./konsol
COPY --from=builder /app/target/site ./site

# get_configuration() reads these at runtime; they must match how the site
# was built (output name and pkg dir) and where it was copied to above.
ENV LEPTOS_OUTPUT_NAME=konsol \
    LEPTOS_SITE_ROOT=./site \
    LEPTOS_SITE_PKG_DIR=konsol/pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:8080

EXPOSE 8080

CMD ["./konsol"]
