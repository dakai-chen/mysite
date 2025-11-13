CREATE TABLE IF NOT EXISTS cache (
    id              TEXT    NOT NULL,
    kind            TEXT    NOT NULL,
    data            TEXT    NOT NULL,
    created_at      INTEGER NOT NULL,
    expires_at      INTEGER NOT NULL,
    PRIMARY KEY (kind, id)
);

CREATE INDEX IF NOT EXISTS idx_expires_at ON cache (expires_at);
