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
- [x] Configuration pattern (zero2prod style)
- [x] Telemetry wired
- [x] Application struct pattern
- [x] Health route confirmed working
- [x] Error handling convention established
- [x] SQLx migrations — urls table created
- [x] DB connection verified at startup
- [~] POST /shorten — in progress
- [ ] GET /{short_code} implemented
- [ ] Structured errors in db layer
- [ ] Integration tests
- [ ] Deployment

## Project Structure
src/
├── main.rs
├── lib.rs
├── startup.rs
├── configuration.rs
├── telemetry.rs
├── models.rs          # Url, CreateUrlRequest, UrlResponse
├── utils.rs           # validate_url(), generate_short_code()
├── routes/
│   ├── mod.rs
│   ├── health.rs
│   ├── shorten.rs     # POST /shorten
│   └── redirect.rs    # GET /{short_code} (coming)
└── db/
    ├── mod.rs
    └── urls.rs        # insert_url(), get_url_by_code() (coming)

configuration/
├── base.yaml
└── production.yaml

## Key Decisions Made
- Short code: random Base62, 7 chars, retry up to 3x on collision → 500
- Redirect: 302 (not 301)
- Auth: deferred to v2
- Custom slugs: in scope for MVP, first-writer-wins → 409
- is_custom column: deferred to v2
- Duplicate long_urls: allowed, two rows created
- Primary key: UUID v4
- Index on short_code
- API: POST /shorten → 201, GET /{short_code} → 302
- Error shape: { error: string, message: string }
- 404 vs 410 for missing vs expired
- PgPool::connect_lazy_with() + eager acquire() check at startup
- db layer has no actix dependency — takes &PgPool directly
- RETURNING * on insert — one round trip

## Error Handling
- application code: anyhow::Error + .context()
- db layer: sqlx::Error for now, thiserror custom enum later
- Rule: anyhow = care about message, thiserror = care about type

## Commands
- cargo run — start server
- curl http://127.0.0.1:8080/health — confirm 200
- sqlx migrate run — run pending migrations
- cargo sqlx prepare — regenerate .sqlx offline cache

## Conventions
- All SQL lives in db/urls.rs
- Routes are thin — logic lives in db layer
- db layer takes &PgPool, never web::Data<AppState>
- validate_url() and generate_short_code() live in utils.rs