# Design Doc: URL Shortener

## Problem Statement
Given a long url, I want users to be able to take that long url and be able to shorten it with possible custom alias. 
This short url can be shared amongst other users and will redirect the user to the original long url.

## Goals 
(Goals should be a small list)
1. A user can submit a long URL and receive a shortened URL in return.
2. A visitor who clicks a short URL is redirected to the original destination.
3. Short codes are unique — no two long URLs share the same short code.
4. The system persists mappings so short links work beyond a single session.

## Non-Goals
No other fancy features like user login and list of urls that a user has created under their profile. (maybe that can be in v2)

## High-Level Architecture
(Step by step detailed description of the happy path, resolution flow, components involved, what handles the HTTP request, )
Creation flow: 
    Client → POST /shorten (long_url, optional custom_slug)
        → validate URL is well-formed
        → generate or accept short code
        → check for collision in DB
        → persist (short_code, long_url, created_at, expires_at) to DB
        → return short URL to client

Resolution flow:
  Client → GET /{short_code}
        → look up short_code in DB
        → if not found → 404
        → if expired → 410 Gone
        → if found → HTTP redirect to long_url

Components:
  - Actix-web HTTP server (handles routing and request lifecycle)
  - PostgreSQL (persists URL mappings)
  - (future) Redis cache (short-circuit DB lookup on hot links)
  - (future) Kafka to handle high request write throughput (releave single service from running hot and spreading load)


## Key Design Decisions & Tradeoffs
Short code generation: 
Using random Base62 (a-z, A-Z, 0-9), 7 characters. On collision, retry with a new random code up to 3 times, then return a 500. SHA256-truncation is an alternative but produces less readable codes and has the same collision problem without being meaningfully safer. Random Base62 at 7 chars gives 62^7 ≈ 3.5 trillion combinations — sufficient for this scale.

Database schema:
CREATE TABLE urls (
  id          UUID PRIMARY KEY,
  short_code  VARCHAR(16) UNIQUE NOT NULL,
  long_url    TEXT NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at  TIMESTAMPTZ
);
CREATE INDEX ON urls (short_code); // because high freq access pattern; every resolution flow does a lookup on this column

301 vs 302:
Using 302 (temporary redirect). A 301 tells browsers and CDNs to cache the redirect permanently — meaning if you ever need to update or expire a link, browsers will keep going to the old destination from cache without hitting your server. 302 ensures every click goes through your server, which is slightly slower but gives you control: you can expire links, track click counts in future, and change the destination. The performance cost is acceptable for this use case.

## Open Questions
(what don't you know yet that could affect the design?)
1. Collision handling: on short code collision, do we retry silently (preferred) 
   or surface an error? Decision needed before implementing generation logic.

2. Custom slugs: if two users request the same custom slug, first writer wins 
   and second gets a 409 Conflict. Is this acceptable UX?

3. Link expiration: do links expire by default (e.g. 30 days) or live forever 
   unless set explicitly? Affects storage growth over time.

4. Rate limiting: not in MVP scope but the system should be designed so it can 
   be added at the middleware layer without touching business logic.

## API Contract

All responses return JSON. Errors follow a consistent shape:
{ "error": "<machine-readable code>", "message": "<human-readable description>" }

---

### POST /shorten

Creates a new short URL mapping.

**Request body:**
{
  "long_url": String,             // required, must be a valid URL
  "alias": Optional<String>       // optional, custom slug (a-z, 0-9, hyphen only)
}

**Success — 201 Created:**
{
  "short_code": String,   // e.g. "abc1234"
  "short_url":  String,   // e.g. "https://tiny.url/abc1234"
  "long_url":   String,   // echo back what was stored
  "expires_at": String    // ISO 8601 timestamp, or null if no expiry
}

**Errors:**
400 Bad Request — long_url is missing or not a valid URL
{ "error": "invalid_url", "message": "long_url must be a valid URL" }

400 Bad Request — alias contains invalid characters
{ "error": "invalid_alias", "message": "alias may only contain a-z, 0-9, and hyphens" }

409 Conflict — requested alias is already taken
{ "error": "alias_conflict", "message": "that alias is already in use" }

500 Internal Server Error — could not generate unique code after retries
{ "error": "generation_failed", "message": "failed to generate a unique short code, try again" }

---

### GET /{short_code}

Resolves a short code and redirects to the original URL.

**Path parameter:**
short_code: String   // the 7-char code or custom alias

**Success — 302 Found:**
Header: Location: <long_url>
Body: empty

**Errors:**
404 Not Found — short_code does not exist
{ "error": "not_found", "message": "no URL found for that code" }

410 Gone — short_code existed but has expired
{ "error": "expired", "message": "this short URL has expired" }