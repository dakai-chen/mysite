CREATE TABLE IF NOT EXISTS article_attachment (
    id              TEXT    PRIMARY KEY,
    article_id      TEXT    NOT NULL,
    resource_id     TEXT    NOT NULL,
    created_at      INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_article_id ON article_attachment (article_id);
