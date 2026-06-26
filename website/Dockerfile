# Build context is the REPO ROOT (see fly.toml / deploy.yml), so this Dockerfile
# can copy both the site app (under website/) and the canonical writing "data"
# (essays/ and assets/ at the repo root) without duplicating the latter.

# ---- builder ----
FROM rust:1-slim-bookworm AS builder
WORKDIR /build

# Cache dependencies first
COPY website/Cargo.toml website/Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build the real binary
COPY website/src ./src
RUN touch src/main.rs && cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim
WORKDIR /app
ENV PORT=8080 \
    BW_SITE_ROOT=/app \
    RUST_LOG=info

COPY --from=builder /build/target/release/site /usr/local/bin/site
# Site app files
COPY website/templates ./templates
COPY website/content ./content
COPY website/static ./static
COPY website/Buddy_Williams_Resume.pdf ./
# Canonical writing data (single source of truth, rendered directly)
COPY essays ./essays
COPY assets ./assets

EXPOSE 8080
CMD ["site"]
