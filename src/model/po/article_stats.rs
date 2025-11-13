/// 文章统计信息
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ArticleStatsPo {
    /// ID
    pub id: String,
    /// 文章ID
    pub article_id: String,
    /// 累计页面访问量
    pub pv: u64,
    /// 累计独立访客数
    pub uv: u64,
}
