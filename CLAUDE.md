# CLAUDE.md — URL Shortener Project

## Stack
- Language: Rust
- Web framework: actix-web 4
- Async runtime: Tokio
- Database: PostgreSQL via SQLx 0.8
- Cache: Redis via deadpool-redis
- Rate limiting: actix-governor (token bucket)

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
- [x] GET /{short_code} implemented and tested
- [x] Custom ApiError type with ResponseError impl
- [x] Integration tests — 7 tests passing
- [x] Rate limiting — actix-governor, per-route scopes
- [x] Redis cache — verified working, graceful degradation confirmed

## Project Structure
src/
├── main.rs
├── lib.rs
├── startup.rs
├── configuration.rs
├── telemetry.rs
├── middleware.rs      # governor config helpers
├── models.rs
├── utils.rs
├── errors.rs
├── cache.rs           # Redis ops — get, set, set_not_found
├── routes/
│   ├── mod.rs
│   ├── health.rs
│   ├── shorten.rs     # #[post("")]
│   └── redirect.rs    # cache-first, DB fallback
└── db/
    ├── mod.rs
    └── urls.rs

tests/
├── api.rs
├── helpers.rs
├── health.rs
├── shorten.rs
└── redirect.rs

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
- cache layer takes &RedisPool, never web::Data<AppState>
- RETURNING * on insert — one round trip
- Alias validation: alphanumeric + hyphens only
- Retry loop: 3 attempts for auto-generated codes
- Expiry check: if let Some(expires_at) pattern
- ApiError: typed error with ResponseError impl
- Handler return type: Result<HttpResponse, ApiError>
- Rate limiting: actix-governor, token bucket
- POST /shorten: 20 req/min per IP (burst 20, refill 1/3s)
- GET /{short_code}: 30 req/min per IP (burst 30, refill 1/2s)
- Governor constructed outside HttpServer closure — shared across workers
- Cache: Redis, key = "url:{short_code}", value = {long_url, expires_at}
- Smart TTL: min(3600, seconds_until_expiry)
- Cache negative results (NOT_FOUND) 60s — prevents cache penetration
- Redis errors swallowed silently — graceful degradation to DB
- &str over String in cache functions — only reading, not owning

## Cache Flow (GET /{short_code})
1. Check Redis → HIT: validate expiry in cached value, serve 302
2. Check Redis → NOT_FOUND: return 404 (no DB query)
3. Check Redis → MISS: query DB
4. DB not found: cache NOT_FOUND 60s, return 404
5. DB found: cache {long_url, expires_at} with smart TTL, return 302

## Resilience
- Redis down → cache miss → fall through to DB → serve normally
- Never propagate Redis errors to user
- DB is always source of truth
- Verified: DB down + warm cache → 302 served correctly

## Connection Pool Pattern
- PgPool and RedisPool live in AppState
- Created once at startup, shared across all requests and workers
- Handlers borrow connections from pool, return after use
- Pool manages connection lifecycle, reconnection, limits

## Error Handling
- application code: anyhow::Error + .context()
- db layer: sqlx::Error bubbled up, matched in handler
- HTTP errors: ApiError with convenience constructors
- Rate limited: 429 Too Many Requests + Retry-After header
- Redis errors: swallowed, treated as miss

## Testing Pattern
- Tests live in tests/ with api.rs as entry point
- spawn_app() creates isolated TestApp per test
- Each test gets its own database: test_{uuid}
- Telemetry init guarded by OnceLock
- reqwest::redirect::Policy::none() for redirect tests
- Direct DB inserts via app.db_pool for test setup
- Verify redirect via Location header

## Commands
- cargo run — start server
- cargo check — verify compilation
- cargo test — run all tests
- sqlx migrate run — run pending migrations
- cargo sqlx prepare — regenerate .sqlx offline cache
- docker run -d -p 6379:6379 redis — start Redis
- docker exec -it <id> redis-cli — inspect Redis
- GET url:{short_code} — verify cache entry in redis-cli

## Conventions
- All SQL lives in db/urls.rs
- All Redis operations live in cache.rs
- Routes are thin — logic in db/cache layers
- db layer takes &PgPool directly
- cache layer takes &RedisPool directly
- Error responses: ApiError
- Always .await async calls
- &str for read-only string params, String for owned