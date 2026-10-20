# AGENTS.md — Techno Penguin Site

This file contains high-signal guidance for working in this repo. Every claim is verified from the codebase.

## Toolchain

- **Rust nightly** is required. Install with:
  ```
  rustup toolchain install nightly --allow-downgrade
  rustup update
  rustup target add wasm32-unknown-unknown
  ```
- `Cargo.toml` features: `ssr` (enables DB/sqlx), `hydrate`, `bin-features = ["ssr"]`, `bin-default-features = false`

## Installation — one-time setup

Run `.env` and install tooling:

```
source .env           # sets DATABASE_URL and other env vars
cargo install cargo-generate
cargo install cargo-leptos --locked
cargo install sqlx-cli
```

## Database

- `.env` defines `DATABASE_URL="sqlite://data/techno_penguin.db"`
- Migrations live in `migrations/`. Run once:
  ```
  source .env
  sqlx database create
  sqlx migrate run --source ./migrations
  ```
- To re-apply migrations after code changes:
  ```
  source .env
  sqlx migrate run --source ./migrations
  ```
- Runtime: `database::init_db()` in `src/database.rs` creates the pool; `database::get_db()` returns it.
- All DB ops are gated behind `#[cfg(feature = "ssr")]`.

## Build & run

```
cargo leptos build     # build the app
cargo leptos watch     # serve + watch for changes
cargo leptos serve     # build + start server
```

App runs at <http://127.0.0.1:3000> (configured in `Cargo.toml` `site-addr` and `.env`).

## Docker

```
docker compose up -d   # build and start the container
```

- Mounts `./data` at `/app/data` for the SQLite DB.
- Sets `LEPTOS_ENV=PROD`, `LEPTOS_SITE_ROOT=/app/site`.

## i18n (leptos-fluent)

- Locales in `./locales/`. Default language: `en`.
- Language switching via `Wgt_LanguageSwitcher` component.
- URL param: `?lang=en|fr`. Persisted to localStorage/cookie.
- Translation files must be valid — `check_translations: true` in `I18nProvider`.

## Tailwind CSS

- Config in `style/tailwind.css`. Import via `style/main.scss` (currently empty; Tailwind runs through the css bundler).
- Input file referenced in `Cargo.toml`: `tailwind-input-file = "style/tailwind.css"`.
- Colours and fonts are defined in `style/tailwind.css` theme block.

## Project structure

- Single package `techno_penguin` (cdylib + rlib).
- `src/app.rs` — shell component.
- `src/main.rs` — Axum router with `generate_route_list(App)`.
- Routes: `/`, `/assembly`, `/training`, `/tune_ups`, `/contact`.
- `src/routes/mod.rs` — exposes `SaveContact` server function.
- `src/components/mod.rs` — composes: `App`, `ContactForm`, `Navbar`, `Footer`, `AssemblyPage`, `TrainingPage`, `TuneUpsPage`, `HomePage`.
- `src/models/contact_submission.rs` — `ContactSubmission` struct with `add()`, `delete()`, `get()`.
- DB: SQLite via sqlx. Migrations in `migrations/1_init_up.sql` / `2_data_up.sql`.

## Server functions (API)

- Contact form POSTs to `/api/SaveContact` (defined in `src/routes/contact_submissions.rs` as `#[server(SaveContact, prefix = "/api", endpoint = "SaveContact")]`).
- On success: renders "send-message-success" green text.
- On error: renders "send-message-failure" red text.

## Contact form notifications (email)

- `src/notifications.rs` emails the site owner on each valid submission (via `lettre`, SMTP). No local mail daemon — mail is relayed by an external SMTP provider.
- Env vars (in `.env`, passed through by `docker-compose.yml`): `SMTP_HOST`, `SMTP_PORT`, `SMTP_USERNAME`, `SMTP_PASSWORD`, `NOTIFY_FROM`, `NOTIFY_TO`.
- Transport security follows the port (`tls_mode_for` in `src/notifications.rs`): `465` = implicit TLS, `25` = plaintext (this site's relay), anything else = STARTTLS (default port `587`).
- Empty `SMTP_HOST` disables notifications; the form keeps working and the skip is logged at `info` level.
- Notification failure is logged (`error`) and never returned to the visitor, because the message is already stored.
- Tests: `cargo test --features ssr --no-default-features` (7 tests, includes notification message building).

## Kill server (local dev)

```
bash kill_site.sh    # stops the process on port 3000
```

## What to avoid

- Do not forget `source .env` before running sqlx or the app — `DATABASE_URL` must be set.
- Do not compile without the `ssr` feature if you need DB access — it's gated by `#[cfg(feature = "ssr")]`.
- Do not edit translation files without regenerating/validating i18n; `check_translations: true` will panic on mismatches.