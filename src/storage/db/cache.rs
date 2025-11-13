use crate::model::po::cache::CachePo;
use crate::storage::db::DbConn;
use crate::util::time::UnixTimestampSecs;

pub async fn create(cache: &CachePo, db: &mut DbConn) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT INTO cache (
            `id`,
            `kind`,
            `data`,
            `created_at`,
            `expires_at`
        ) VALUES (?, ?, ?, ?, ?)
        ",
    )
    .bind(&cache.id)
    .bind(&cache.kind)
    .bind(&cache.data)
    .bind(&cache.created_at)
    .bind(&cache.expires_at)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(From::from)
}

pub async fn update(cache: &CachePo, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query(
        "
        UPDATE cache SET
            `data` = ?,
            `created_at` = ?,
            `expires_at` = ?
        WHERE
            `kind` = ? AND `id` = ?
        ",
    )
    .bind(&cache.data)
    .bind(&cache.created_at)
    .bind(&cache.expires_at)
    .bind(&cache.kind)
    .bind(&cache.id)
    .execute(db)
    .await
    .map(|res| res.rows_affected())
    .map_err(From::from)
}

pub async fn update_active(cache: &CachePo, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query(
        "
        UPDATE cache SET
            `data` = ?,
            `created_at` = ?,
            `expires_at` = ?
        WHERE
            `kind` = ? AND `id` = ? AND expires_at >= ?
        ",
    )
    .bind(&cache.data)
    .bind(&cache.created_at)
    .bind(&cache.expires_at)
    .bind(&cache.kind)
    .bind(&cache.id)
    .bind(UnixTimestampSecs::now().as_i64())
    .execute(db)
    .await
    .map(|res| res.rows_affected())
    .map_err(From::from)
}

pub async fn update_active_expires_at(
    kind: &str,
    id: &str,
    expires_at: i64,
    db: &mut DbConn,
) -> anyhow::Result<u64> {
    sqlx::query(
        "
        UPDATE cache SET
            `expires_at` = ?
        WHERE
            `kind` = ? AND `id` = ? AND expires_at >= ?
        ",
    )
    .bind(expires_at)
    .bind(kind)
    .bind(id)
    .bind(UnixTimestampSecs::now().as_i64())
    .execute(db)
    .await
    .map(|res| res.rows_affected())
    .map_err(From::from)
}

pub async fn create_or_update(cache: &CachePo, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query(
        "
        INSERT INTO cache (
            `id`,
            `kind`,
            `data`,
            `created_at`,
            `expires_at`
        ) VALUES (?, ?, ?, ?, ?)
        ON CONFLICT (`kind`, `id`) DO UPDATE SET
            `data` = excluded.data,
            `created_at` = excluded.created_at,
            `expires_at` = excluded.expires_at
        ",
    )
    .bind(&cache.id)
    .bind(&cache.kind)
    .bind(&cache.data)
    .bind(&cache.created_at)
    .bind(&cache.expires_at)
    .execute(db)
    .await
    .map(|res| res.rows_affected())
    .map_err(From::from)
}

pub async fn remove(kind: &str, id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM cache WHERE kind = ? AND id = ?")
        .bind(kind)
        .bind(id)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn remove_by_id_prefix(
    kind: &str,
    id_prefix: &str,
    db: &mut DbConn,
) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM cache WHERE kind = ? AND id LIKE CONCAT(?, '%')")
        .bind(kind)
        .bind(crate::util::sqlx::escape_like_special_chars(id_prefix))
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn remove_single_expired(kind: &str, id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM cache WHERE kind = ? AND id = ? AND expires_at < ?")
        .bind(kind)
        .bind(id)
        .bind(UnixTimestampSecs::now().as_i64())
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn remove_all_expired(limit: u64, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM cache WHERE rowid IN (SELECT rowid FROM cache WHERE expires_at < ? ORDER BY expires_at LIMIT ?)")
        .bind(UnixTimestampSecs::now().as_i64())
        .bind(i64::try_from(limit)?)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn find_active(kind: &str, id: &str, db: &mut DbConn) -> anyhow::Result<Option<CachePo>> {
    sqlx::query_as("SELECT * FROM cache WHERE kind = ? AND id = ? AND expires_at >= ?")
        .bind(kind)
        .bind(id)
        .bind(UnixTimestampSecs::now().as_i64())
        .fetch_optional(db)
        .await
        .map_err(From::from)
}

pub async fn exists_active(kind: &str, id: &str, db: &mut DbConn) -> anyhow::Result<bool> {
    sqlx::query_scalar("SELECT COUNT(*) FROM cache WHERE kind = ? AND id = ? AND expires_at >= ?")
        .bind(kind)
        .bind(id)
        .bind(UnixTimestampSecs::now().as_i64())
        .fetch_one(db)
        .await
        .map(|count: i64| count != 0)
        .map_err(From::from)
}

pub async fn get_active_expires_at(
    kind: &str,
    id: &str,
    db: &mut DbConn,
) -> anyhow::Result<Option<i64>> {
    sqlx::query_scalar("SELECT expires_at FROM cache WHERE kind = ? AND id = ? AND expires_at >= ?")
        .bind(kind)
        .bind(id)
        .bind(UnixTimestampSecs::now().as_i64())
        .fetch_optional(db)
        .await
        .map_err(From::from)
}
