-- Add migration script here

CREATE TABLE urls (
  id          UUID PRIMARY KEY,
  short_code  VARCHAR(16) UNIQUE NOT NULL,
  long_url    TEXT NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at  TIMESTAMPTZ
);
CREATE INDEX idx_urls_short_code ON urls (short_code);