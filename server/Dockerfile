# Stage 1 — build the Rust binary
# Note: first build is slow (~5-10 min) because all crates compile from scratch.
# Subsequent builds with unchanged source reuse Docker layer cache.
FROM docker.io/library/rust:slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2 — minimal runtime image
FROM docker.io/library/debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/server
EXPOSE 5000
CMD ["server"]
