/// 文章附件
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ArticleAttachmentPo {
    /// 附件ID
    pub id: String,
    /// 文章ID
    pub article_id: String,
    /// 资源文件ID
    pub resource_id: String,
    /// 创建时间
    pub created_at: i64,
}
