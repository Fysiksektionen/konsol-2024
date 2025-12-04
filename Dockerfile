# Build frontends
FROM node:20 AS frontend-builder
ARG VITE_GOOGLE_CLIENT_ID

ENV VITE_GOOGLE_CLIENT_ID=$VITE_GOOGLE_CLIENT_ID
ENV VITE_API_BASE_URL=../api

WORKDIR /work

# copy frontends (build context must be repo root)
COPY admin-frontend ./admin-frontend
WORKDIR /work/admin-frontend
RUN npm ci --prefer-offline --no-audit --silent && npm run build --silent \
    && mkdir -p /out/admin \
    && (cp -r build/* /out/admin 2>/dev/null || cp -r dist/* /out/admin 2>/dev/null)

WORKDIR /work
COPY screen-frontend ./screen-frontend
WORKDIR /work/screen-frontend
# Uncomment when screen frontend is ready
RUN npm ci --prefer-offline --no-audit --silent && npm run build --silent \
    && mkdir -p /out/screen \
    && (cp -r build/* /out/screen 2>/dev/null || cp -r dist/* /out/screen 2>/dev/null)

FROM debian:bookworm-slim AS runtime
WORKDIR /app

# runtime deps (nginx, sqlite runtime, openssl runtime, ca certs)
RUN apt-get update && apt-get install -y \
    nginx \
    ca-certificates \
    sqlite3 \
    libsqlite3-0 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# copy built frontends into places nginx will serve
RUN mkdir -p /var/www/admin /var/www/screen
COPY --from=frontend-builder /out/admin /var/www/admin
# Uncomment when screen frontend is ready
COPY --from=frontend-builder /out/screen /var/www/screen

COPY nginx.conf /etc/nginx/conf.d/app.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
