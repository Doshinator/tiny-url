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
- [x] Configuration pattern (zero2prod style, base/local/production yaml)
- [x] Telemetry wired (tracing + bunyan + tracing-actix-web)
- [x] Application struct pattern (build + run_until_stopped)
- [ ] Health route confirmed working (cargo run → GET /health → 200)
- [ ] SQLx migrations setup
- [ ] DB pool confirmed connected
- [ ] POST /shorten implemented
- [ ] GET /{short_code} implemented
- [ ] Error handling + structured errors
- [ ] Integration tests
- [ ] Deployment

## Project Structure
src/
├── main.rs          # entry point only — thin
├── lib.rs           # pub mod declarations
├── startup.rs       # Application struct, build(), run()
├── configuration.rs # Settings structs, get_configuration()
├── telemetry.rs     # get_subscriber(), init_subscriber()
├── routes/
│   ├── mod.rs
│   └── health.rs    # GET /health → 200
└── (coming) db/, models.rs

configuration/
├── base.yaml        # all base settings
├── local.yaml       # local overrides (to simplify/remove)
└── production.yaml  # production overrides (env vars, no ${ })

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

## Known Issues to Fix
- Environment::as_str() returns capitalized strings → breaks yaml filenames
- production.yaml uses ${} syntax which config crate doesn't interpolate
- local.yaml is redundant with base.yaml — simplify

## Configuration Pattern
- Uses `config` crate
- base.yaml → all shared/default settings
- production.yaml → production overrides
- APP_ENVIRONMENT env var selects environment (default: local)
- APP__ prefix for env var overrides (e.g. APP__DATABASE__PASSWORD)

## Commands
(fill in once health check confirmed working)

## Conventions
- Errors: thiserror for library errors, anyhow for application errors
- All SQL lives in db/urls.rs
- Routes are thin — logic lives in db layer