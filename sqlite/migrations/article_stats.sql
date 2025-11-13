CREATE TABLE IF NOT EXISTS article_stats (
    id              TEXT    PRIMARY KEY,
    article_id      TEXT    NOT NULL,
    pv              INTEGER NOT NULL,
    uv              INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_article_id ON article_stats (article_id);
