# ---- builder ----
FROM rust:1-slim-bookworm AS builder
WORKDIR /build

# Cache dependencies first
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build the real binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim
WORKDIR /app
ENV PORT=8080 \
    BW_SITE_ROOT=/app \
    RUST_LOG=info

COPY --from=builder /build/target/release/site /usr/local/bin/site
COPY templates ./templates
COPY content ./content
COPY static ./static
COPY Buddy_Williams_Resume.pdf ./

EXPOSE 8080
CMD ["site"]
