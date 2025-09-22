# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: Axum server setup and routes.
- `src/routes/*`: HTTP handlers (`health`, `video`, `database`, `ner_route`, `errors`).
- `src/db/*`: Database connection, models, and repositories.
- `src/ai/*`: AI-related helpers (NER request client).
- `migrations/`: SQL schema files; `docker-compose.yml`: local Postgres.
- `.env`: local config (e.g., `DATABASE_URL`).

## Build, Test, and Development Commands
- `docker-compose up -d postgres`: start local Postgres 15.
- `cargo build`: compile the server.
- `cargo run`: run on `0.0.0.0:8000`.
- `cargo test`: run unit/integration tests.
- `cargo fmt` / `cargo clippy --all-targets -- -D warnings`: format and lint.
- Example: `curl http://localhost:8000/health` → `{\"status\":\"healthy\"...}`.

## Coding Style & Naming Conventions
- Rust 2024 edition; idiomatic Rust style via `rustfmt`.
- Indentation: 4 spaces; line length ~100 where practical.
- Naming: modules `snake_case`, types `UpperCamelCase`, functions/vars `snake_case`.
- Error handling: prefer `thiserror`-style enums; here use `routes::errors::AppError`.
- HTTP routes live under `src/routes`; keep request/response types near handlers.

## Testing Guidelines
- Unit tests inline with modules using `#[cfg(test)] mod tests { ... }`.
- Integration tests in `tests/` (one file per feature, descriptive names).
- Run all tests with `cargo test`; add minimal DB stubs or use a test database.
- Prefer deterministic tests; assert on shapes/fields, not timestamps.

## Commit & Pull Request Guidelines
- Commit format: Conventional Commits (e.g., `feat(routes): add NER endpoint`).
  - The history shows `FEAT:` used; prefer lowercase (`feat:`) going forward.
- PRs must include: concise description, linked issues, testing notes, and curl examples.
- Keep changes focused and small; update docs (`readme.md`, this file) as needed.

## Security & Configuration Tips
- Configure `DATABASE_URL` (see `.env` for local default).
- Start Postgres before running the server. For schema reset, POST `http://localhost:8000/reset-database`.
- NER calls expect an AI service at `http://localhost:8080/ner`.
- Do not commit secrets; `.env` is local-only. Prefer env vars in CI.

