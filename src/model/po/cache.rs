/// 缓存
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CachePo {
    /// ID
    pub id: String,
    /// 缓存类型
    pub kind: String,
    /// 缓存数据
    pub data: String,
    /// 创建时间
    pub created_at: i64,
    /// 过期时间
    pub expires_at: i64,
}
