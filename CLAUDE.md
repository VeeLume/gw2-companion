# GW2 Companion

A Guild Wars 2 companion application built with Tauri + SvelteKit on the veelume-ui kit.

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

- **`crates/gw2-mystic-forge`** — Mystic Forge recipes as compiled-in static data (the official
  API has none; every legendary's last step is one)
  - `recipes()` / `get(id)` / `by_output(item)` over `static` data in `src/generated.rs`:
    no parsing or allocation at runtime. Types and `FORMAT_VERSION` in `src/lib.rs`
  - `data/mystic_forge.json` is the same data as reviewable JSON; `data/excluded.json` lists
    every dropped wiki record with its reason
  - `Yield::Fixed` is the only yield a plan may rely on; `Random` recipes are listed but not usable
  - All three files are generated, never hand-edited: `cargo run -p gw2-mystic-forge --features
    generate --bin generate-mystic-forge` queries the GW2 wiki's Semantic MediaWiki data,
    validates every id against `/v2/items` and stamps the game build. Regenerate when
    `/v2/build` moves past `source().game_build`

- **`src-tauri/`** — Tauri v2 backend
  - Tauri commands exposing API + DB to the frontend
  - State management via Tauri's managed state (`AppState`: DB, public client, optional auth
    client, settings snapshot)
  - **Typed IPC via tauri-specta**: every command carries `#[specta::specta]` and is listed in
    `specta_builder()` in `lib.rs`; debug starts regenerate `src/lib/bindings.ts` (or
    `cargo run -p gw2-companion-app --features bindgen --bin export-bindings`). A command not
    in that list does not exist for the frontend. specta/tauri-specta are pinned exactly.
  - gw2-api types have no `specta::Type`, so they never cross IPC directly: commands return the
    app-owned views in `dto.rs` (enums as API strings, coin as `f64` copper — specta rejects `i64`)
  - App preferences: one `AppSettings` snapshot in `settings.json` (`settings.rs`/`store.rs`,
    whole-snapshot save). The API key is NOT in it; it lives in the DB `settings` table.
    Theme/density live in the frontend's localStorage.
  - Auto-updater (desktop): `tauri-plugin-updater` + `tauri-plugin-process`; `pubkey` in
    `tauri.conf.json` is empty until a signing key exists, and `*.key` is gitignored

- **`src/`** — Svelte 5 + SvelteKit frontend (adapter-static, SPA, output in `build/`)
  - Built on `@veelume/ui` (VeeLume/veelume-ui), git-installed and pinned by tag in `package.json`.
    Its rulebook is `packages/ui/CLAUDE.md` in `../veelume-ui` — read it before building surfaces.
    Kit fixes go into the kit and a tag bump, never a local copy.
  - Shell (`Shell.Root`/rail/bottom bar) in `routes/+layout.svelte`; destinations are data in
    `lib/nav.svelte.ts`, settings categories in `lib/settingsNav.ts`
  - `bits-ui` must stay pinned to the same exact version as the kit's
  - `vite.config.ts` excludes the kit from `optimizeDeps` (it ships `.svelte.ts` source)
  - Calls Rust through the generated `commands` in `src/lib/bindings.ts`; `src/lib/api.ts` holds
    only helpers (`unwrap`, `formatCoin`). Stores: `settings`, `updater`, `appearance`

## Features (v1)

1. **Legendary Crafting Tracker** — Track materials, precursors, and crafting progress
2. **Achievement Tracker** — Browse achievements with dependency trees
3. **Trading Post Tools** — Price lookups, flipping analysis
4. **Daily/Weekly Checklist** — Dailies, weeklies, wizard's vault tasks

## Conventions

- `anyhow` in the Tauri app layer
- Tauri commands are `async` and return `Result<T, String>` (Tauri convention)
- Frontend uses TypeScript strict mode
- CSS via Tailwind CSS v4; theme tokens in `src/theme.css` (shadcn token names), structure in `src/app.css`
- Never hand-write TypeScript mirrors of Rust types; add a view in `dto.rs` and regenerate the bindings
- Tauri npm packages and Rust crates must share major.minor (`pnpm tauri info` flags a mismatch,
  and `tauri build` refuses it); pnpm holds back day-old releases, so bump both sides together

## Development

```bash
# Install frontend deps
pnpm install

# Type-check the frontend
pnpm check

# Run dev mode (starts both Tauri + Vite)
cargo tauri dev

# Build for release
cargo tauri build

# Tests (gw2-api's own tests run in ../gw2-api)
cargo test
```
