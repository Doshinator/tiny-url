# CLAUDE.md — URL Shortener Project

## Stack
- Language: Rust
- Web framework: actix-web
- Database: PostgreSQL
- (future) Cache: Redis
- (future) Queue: Kafka (high write throughput)

## Project Status
- [x] Problem statement defined
- [x] Design doc written
- [x] Architecture defined (creation + resolution flows)
- [x] Key design decisions made
- [x] API contract defined
- [ ] DB schema finalized in code
- [ ] Project scaffolded
- [ ] Core endpoints implemented
- [ ] Error handling + structured logging
- [ ] Integration tests
- [ ] Deployment

## Key Decisions Made
- Short code: random Base62, 7 chars, retry up to 3x on collision → 500
- Redirect: 302 (not 301) — preserves server control over expiry/tracking
- Auth: deferred to v2, MVP is anonymous
- Custom slugs: in scope for MVP, first-writer-wins on conflict → 409
- Primary key: UUID (not BIGSERIAL)
- Index on short_code — high frequency lookup column
- API contract finalized: POST /shorten → 201, GET /{short_code} → 302
- Consistent error shape: { error: string, message: string } on all failures
- 201 (not 200) for resource creation
- 404 vs 410 distinction enforced for missing vs expired links

## Open Questions (unresolved)
- Silent retry vs surfaced error on collision → leaning silent retry
- Link expiration: default 30 days vs explicit only → undecided
- Rate limiting: deferred to post-MVP, designed to slot in at middleware layer

## Conventions
(to be filled as we establish them)

## Commands
(to be filled once project is scaffolded)
