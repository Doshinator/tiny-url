# CLAUDE.md — URL Shortener Project

## Stack
- Language: Rust
- Web framework: actix-web 4
- Async runtime: Tokio
- Database: PostgreSQL via SQLx 0.8
- (future) Cache: Redis
- (future) Queue: Kafka

## Project Status
- [x] Problem statement defined
- [x] Design doc written
- [x] Architecture defined (creation + resolution flows)
- [x] Key design decisions made
- [x] API contract defined
- [x] Cargo.toml dependencies defined
- [x] Project structure designed
- [x] Configuration pattern (zero2prod style, base/production yaml)
- [x] Telemetry wired (tracing + bunyan + tracing-actix-web)
- [x] Application struct pattern (build + run_until_stopped)
- [x] Health route confirmed working (curl → 200)
- [x] Error handling in startup.rs (anyhow + context)
- [ ] SQLx migrations setup
- [ ] DB pool confirmed connected
- [ ] POST /shorten implemented
- [ ] GET /{short_code} implemented
- [ ] Structured errors in db layer (thiserror)
- [ ] Integration tests
- [ ] Deployment

## Error Handling Convention
- application code (startup, handlers): anyhow::Error + .context()
- db/library code: thiserror custom enum so callers can match variants
- Rule: anyhow = care about message, thiserror = care about type

## Project Structure
src/
├── main.rs
├── lib.rs
├── startup.rs       # Application struct — now returns anyhow::Error
├── configuration.rs
├── telemetry.rs
├── routes/
│   ├── mod.rs
│   └── health.rs
└── (coming) db/, models.rs

configuration/
├── base.yaml
└── production.yaml

## Key Decisions Made
- Short code: random Base62, 7 chars, retry up to 3x on collision → 500
- Redirect: 302 (not 301) — preserves server control
- Auth: deferred to v2, MVP is anonymous
- Custom slugs: in scope for MVP, first-writer-wins → 409
- Primary key: UUID v4
- Index on short_code — high frequency lookup column
- API: POST /shorten → 201, GET /{short_code} → 302
- Error shape: { error: string, message: string }
- 404 vs 410 for missing vs expired links
- PgPool::connect_lazy_with() — lazy connection, fast startup
- Secret<String> for DB password — secrecy crate
- PgConnectOptions over raw connection string
- startup.rs uses anyhow, db layer will use thiserror

## Configuration Pattern
- base.yaml → all default settings
- production.yaml → production overrides (no ${} syntax, use env vars)
- APP_ENVIRONMENT selects environment (default: local)
- APP__DATABASE__PASSWORD etc for env var overrides

## Commands
- cargo run — start server
- curl http://127.0.0.1:8080/health — confirm 200

## Conventions
- All SQL lives in db/urls.rs
- Routes are thin — logic lives in db layer
- .context() on every ? in application code