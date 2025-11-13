use crate::model::po::article_attachment::ArticleAttachmentPo;
use crate::storage::db::DbConn;

pub async fn create(attachment: &ArticleAttachmentPo, db: &mut DbConn) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT INTO article_attachment (
            `id`,
            `article_id`,
            `resource_id`,
            `created_at`
        ) VALUES (?, ?, ?, ?)
        ",
    )
    .bind(&attachment.id)
    .bind(&attachment.article_id)
    .bind(&attachment.resource_id)
    .bind(&attachment.created_at)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(From::from)
}

pub async fn remove(id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM article_attachment WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn remove_by_article_id(article_id: &str, db: &mut DbConn) -> anyhow::Result<u64> {
    sqlx::query("DELETE FROM article_attachment WHERE article_id = ?")
        .bind(article_id)
        .execute(db)
        .await
        .map(|res| res.rows_affected())
        .map_err(From::from)
}

pub async fn list_by_article_id(
    article_id: &str,
    db: &mut DbConn,
) -> anyhow::Result<Vec<ArticleAttachmentPo>> {
    sqlx::query_as("SELECT * FROM article_attachment WHERE article_id = ?")
        .bind(article_id)
        .fetch_all(db)
        .await
        .map_err(From::from)
}

pub async fn find(id: &str, db: &mut DbConn) -> anyhow::Result<Option<ArticleAttachmentPo>> {
    sqlx::query_as("SELECT * FROM article_attachment WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(From::from)
}
