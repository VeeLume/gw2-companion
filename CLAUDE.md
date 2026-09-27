# GW2 Companion

A Guild Wars 2 companion application built with Tauri + SolidJS.

## Architecture

- **`gw2-api`** — lives in its own repo, `../gw2-api` (VeeLume/gw2-api), used as a path dependency
  (`gw2-api = { path = "../gw2-api/crates/gw2-api" }` in the workspace `Cargo.toml`).
  The endpoint macros, the developer CLI and the API docs are there; see its `CLAUDE.md`.
  - All clients share one process-wide rate limiter and static-data cache by default.
    Derive the authenticated client from the public one with `public_client.authenticate(key)`.

- **`crates/gw2-db`** — SQLite persistence layer
  - rusqlite with bundled SQLite, WAL, migrations via `PRAGMA user_version`
  - Schema in `migrations/001_initial.sql` (cache tables, legendary goals, checklists, settings);
    the model modules are still stubs. Only `settings` (API key) is used so far.

- **`src-tauri/`** — Tauri v2 backend
  - Tauri commands exposing API + DB to the frontend
  - State management via Tauri's managed state (`AppState`: DB, public client, optional auth client)

- **`src/`** — SolidJS + TypeScript frontend
  - Vite-based build
  - Communicates with Rust backend via `@tauri-apps/api`

## Features (v1)

1. **Legendary Crafting Tracker** — Track materials, precursors, and crafting progress
2. **Achievement Tracker** — Browse achievements with dependency trees
3. **Trading Post Tools** — Price lookups, flipping analysis
4. **Daily/Weekly Checklist** — Dailies, weeklies, wizard's vault tasks

## Conventions

- `anyhow` in the Tauri app layer
- Tauri commands are `async` and return `Result<T, String>` (Tauri convention)
- Frontend uses TypeScript strict mode
- CSS via Tailwind CSS
- TypeScript mirrors of API types (`src/lib/api.ts`) must match the serialized Rust types in gw2-api

## Development

```bash
# Install frontend deps
npm install

# Run dev mode (starts both Tauri + Vite)
cargo tauri dev

# Build for release
cargo tauri build

# Tests (gw2-api's own tests run in ../gw2-api)
cargo test
```
