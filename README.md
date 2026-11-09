# discord_link_spotify_api

## How to run

### Manual build

```bash
cp .env.example .env
cargo build --release
./target/release/discord_link_spotify_api
```

### Build with docker

```bash
cp .env.example .env
docker compose --env-file .env up -d --build
```

## Requirements

- Rustc
- PostgreSQL
