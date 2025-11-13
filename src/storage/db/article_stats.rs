use crate::model::po::article_stats::ArticleStatsPo;
use crate::storage::db::DbConn;

pub async fn create(stats: &ArticleStatsPo, db: &mut DbConn) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT INTO article_stats (
            `id`,
            `article_id`,
            `pv`,
            `uv`
        ) VALUES (?, ?, ?, ?)
        ",
    )
    .bind(&stats.id)
    .bind(&stats.article_id)
    .bind(&i64::try_from(stats.pv)?)
    .bind(&i64::try_from(stats.uv)?)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(From::from)
}

pub async fn update(stats: &ArticleStatsPo, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query(
        "
        UPDATE article_stats SET
            `article_id` = ?,
            `pv` = ?,
            `uv` = ?
        WHERE
            `id` = ?
        ",
    )
    .bind(&stats.article_id)
    .bind(&i64::try_from(stats.pv)?)
    .bind(&i64::try_from(stats.uv)?)
    .bind(&stats.id)
    .execute(db)
    .await
    .map(|res| res.rows_affected())
    .map_err(From::from)
}

pub async fn remove(id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM article_stats WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn remove_by_article_id(article_id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM article_stats WHERE article_id = ?")
        .bind(article_id)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn find_by_article_id(
    article_id: &str,
    db: &mut DbConn,
) -> anyhow::Result<Option<ArticleStatsPo>> {
    sqlx::query_as("SELECT * FROM article_stats WHERE article_id = ?")
        .bind(article_id)
        .fetch_optional(db)
        .await
        .map_err(From::from)
}

pub async fn increment_by_article_id(
    article_id: &str,
    pv_add: u64,
    uv_add: u64,
    db: &mut DbConn,
) -> anyhow::Result<u64> {
    sqlx::query("UPDATE article_stats SET pv = pv + ?, uv = uv + ? WHERE article_id = ?")
        .bind(&i64::try_from(pv_add)?)
        .bind(&i64::try_from(uv_add)?)
        .bind(article_id)
        .execute(db)
        .await
        .map(|r| r.rows_affected())
        .map_err(Into::into)
}
