# ---------- builder ----------
FROM rust:1.95-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    libclang-dev \
    pkg-config \
    libsqlite3-dev \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
COPY toasty ./toasty
COPY Toasty.toml ./

RUN cargo build --release \
    --bin discord_link_spotify_api \
    --bin db

# ---------- runtime ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --uid 10001 --create-home appuser

WORKDIR /app

COPY --from=builder /app/target/release/discord_link_spotify_api /usr/local/bin/discord_link_spotify_api
COPY --from=builder /app/target/release/db /usr/local/bin/db

COPY --from=builder /app/toasty ./toasty
COPY --from=builder /app/Toasty.toml ./Toasty.toml

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

CMD ["discord_link_spotify_api"]
