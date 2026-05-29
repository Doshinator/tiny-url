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
- [x] Architecture defined
- [x] Key design decisions made
- [x] API contract defined
- [x] Cargo.toml dependencies defined
- [x] Project structure designed
- [x] Configuration pattern
- [x] Telemetry wired
- [x] Application struct pattern
- [x] Health route confirmed working
- [x] Error handling convention established
- [x] SQLx migrations — urls table created
- [x] DB connection verified at startup
- [x] POST /shorten implemented and tested
- [x] GET /{short_code} implemented and tested end to end
- [~] Custom ApiError type — in progress
- [ ] Integration tests
- [ ] Deployment

## Project Structure
src/
├── main.rs
├── lib.rs
├── startup.rs
├── configuration.rs
├── telemetry.rs
├── models.rs
├── utils.rs
├── errors.rs          # ApiError, ResponseError impl
├── routes/
│   ├── mod.rs
│   ├── health.rs
│   ├── shorten.rs
│   └── redirect.rs
└── db/
    ├── mod.rs
    └── urls.rs

configuration/
├── base.yaml
└── production.yaml

## Key Decisions Made
- Short code: random Base62, 7 chars, retry up to 3x on collision → 500
- Redirect: 302 (not 301)
- Auth: deferred to v2
- Custom slugs: MVP, first-writer-wins → 409
- is_custom column: deferred to v2
- Duplicate long_urls: allowed
- Primary key: UUID v4
- Index on short_code
- API: POST /shorten → 201, GET /{short_code} → 302
- Error shape: { error: string, message: string }
- 404 vs 410 for missing vs expired
- db layer takes &PgPool, never web::Data<AppState>
- RETURNING * on insert — one round trip
- Alias validation: alphanumeric + hyphens only
- Retry loop: 3 attempts for auto-generated codes
- Expiry check: if let Some(expires_at) pattern
- ApiError: typed error with ResponseError impl, status skipped in JSON

## Error Handling
- application code: anyhow::Error + .context()
- db layer: sqlx::Error bubbled up, matched in handler
- HTTP errors: ApiError with convenience constructors
- Unique constraint name: urls_short_code_key
- Handler return type: Result<HttpResponse, ApiError>

## Commands
- cargo run — start server
- cargo check — verify compilation
- sqlx migrate run — run pending migrations
- cargo sqlx prepare — regenerate .sqlx offline cache
- curl -X POST http://127.0.0.1:8080/shorten \
    -H "Content-Type: application/json" \
    -d '{"long_url": "https://www.google.com"}'
- curl -v http://127.0.0.1:8080/{short_code}

## Conventions
- All SQL lives in db/urls.rs
- Routes are thin — logic lives in db layer
- db layer takes &PgPool, never web::Data<AppState>
- validate_url() and generate_short_code() in utils.rs
- Error responses: ApiError — never raw serde_json::json! for errors
- Always .await async calls
- Handler return type: Result<HttpResponse, ApiError>