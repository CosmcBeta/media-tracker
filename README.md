# Atlas Media Tracker

A self-hosted media tracker for movies, shows, games, albums, and artists built to run entirely on your own infrastructure.

Organize everything into custom lists, track progress on what you're watching/playing/listening to, and search across multiple media types with metadata pulled live from TMDB, IGDB, and MusicBrainz.

## Features

- **Lists** — create custom lists (watchlists, backlogs, favorites, etc.) with icons
- **Multi-type search** — debounced search across movies, shows, games, albums, and artists
- **Rich item details** — media-type-specific metadata views for each category
- **Progress tracking** — log and review progress history per item
- **Fully self-hosted** — your data, your server, no third-party accounts required

## Tech Stack

**Backend**
- [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx) with PostgreSQL
- External metadata sources: TMDB, IGDB, MusicBrainz

**Frontend**
- React + TypeScript + Vite
- [TanStack Query](https://tanstack.com/query) for data fetching/caching
- [shadcn/ui](https://ui.shadcn.com/) components (Base UI primitives)
- Tailwind CSS
- Tested with Vitest, React Testing Library, and MSW

**Infrastructure**
- Docker Compose (PostgreSQL, Rust backend, nginx-served frontend)
- nginx reverse proxy for API routing

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- API credentials for [TMDB](https://www.themoviedb.org/settings/api) and [IGDB](https://api-docs.igdb.com/#getting-started) (MusicBrainz requires no key)

### Setup

1. Clone the repo:
   ```bash
   git clone https://github.com/CosmcBeta/atlas-media-tracker.git
   cd atlas-media-tracker
   ```

2. Copy the example environment files and fill in your own values:
   ```bash
   cp .env.example .env
   cp backend/.env.example backend/.env
   ```

3. Build and start the stack:
   ```bash
   docker compose up --build
   ```

4. Open the app:
   - **Frontend:** [http://localhost:8080](http://localhost:8080)
   - **Backend API:** [http://localhost:3000/api/v1](http://localhost:3000/api/v1)

### Local Development (without Docker)

**Backend**
```bash
cd backend
cargo run
```

**Frontend**
```bash
cd frontend
pnpm install
pnpm run dev
```

## Testing

**Backend**
```bash
cd backend
cargo test
```

**Frontend**
```bash
cd frontend
pnpm test
```

## Deployment

The Docker Compose setup here is portable to any server. To run behind a custom domain with HTTPS, put a reverse proxy (e.g. Caddy, Traefik, or nginx + certbot) in front of the `frontend` service on your host — no changes needed to this repo's Docker configuration.

## License

This project is licensed under the [MIT License](LICENSE).
