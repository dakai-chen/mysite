use std::time::Duration;

use boluo::data::Extension;
use boluo::extract::FromRequest;
use boluo::request::Request;
use serde::{Deserialize, Serialize};

use crate::context::visitor::VisitorId;
use crate::error::{AppError, AppErrorMeta};
use crate::model::co::article::VisitorArticleAccessPermitCo;
use crate::model::co::visitor::VisitorCo;
use crate::storage::cache::storage::CacheSetMode;
use crate::storage::cache::{Cache, CacheData};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisitorBo {
    visitor: Cache<VisitorCo>,
}

impl VisitorBo {
    pub const VISITOR_TTL: Duration = Duration::from_secs(3600 * 24 * 7);
    pub const VISITOR_KEEP_THRESHOLD: Duration = Duration::from_secs(3600 * 24);

    pub async fn create_and_cache() -> anyhow::Result<Self> {
        let data = VisitorCo {
            visitor_id: crate::util::uuid::v4(),
        };
        let visitor = data.with_ttl(Self::VISITOR_TTL);
        visitor.set(CacheSetMode::OnlyIfNotExists).await?;
        // 删除此访客残留的文章访问许可
        Self::cleanup_article(&visitor.data.visitor_id).await?;
        Ok(Self { visitor })
    }

    pub async fn from_cache(visitor_id: &str) -> anyhow::Result<Option<Self>> {
        Ok(Cache::get(visitor_id)
            .await?
            .map(|cache| Self { visitor: cache }))
    }

    pub fn visitor_id(&self) -> &str {
        &self.visitor.data.visitor_id
    }

    pub fn created_at(&self) -> i64 {
        self.visitor.created_at
    }

    pub fn expires_at(&self) -> i64 {
        self.visitor.expires_at
    }

    pub async fn add_article(&self, article_id: &str) -> anyhow::Result<()> {
        let data = VisitorArticleAccessPermitCo {
            visitor_id: self.visitor_id().into(),
            article_id: article_id.into(),
        };
        let permit = data.with_ttl(crate::config::get().article.access_access_ttl);
        permit.set(CacheSetMode::Overwrite).await?;
        Ok(())
    }

    pub async fn has_article(&self, article_id: &str) -> anyhow::Result<bool> {
        let data = VisitorArticleAccessPermitCo {
            visitor_id: self.visitor_id().into(),
            article_id: article_id.into(),
        };
        Cache::<VisitorArticleAccessPermitCo>::exists(data.generate_id().as_ref()).await
    }

    pub async fn keep(visitor_id: &str) -> anyhow::Result<bool> {
        let Some(ttl) = Cache::<VisitorCo>::get_ttl(visitor_id).await? else {
            return Ok(false);
        };
        if ttl <= Self::VISITOR_KEEP_THRESHOLD {
            Cache::<VisitorCo>::set_ttl(visitor_id, Self::VISITOR_TTL).await
        } else {
            Ok(true)
        }
    }

    async fn cleanup_article(visitor_id: &str) -> anyhow::Result<()> {
        Cache::<VisitorArticleAccessPermitCo>::batch_remove(visitor_id)
            .await
            .map_err(From::from)
    }
}

impl FromRequest for VisitorBo {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        let Some(visitor) = Option::<Extension<VisitorId>>::from_request(request).await? else {
            return Err(AppErrorMeta::Internal.with_context("请求扩展中 VisitorId 不存在"));
        };
        let Some(visitor) = VisitorBo::from_cache(visitor.visitor_id()).await? else {
            return Err(AppErrorMeta::Internal.with_context(format!(
                "缓存中没有找到访客信息，访客ID: {}",
                visitor.visitor_id()
            )));
        };
        Ok(visitor)
    }
}
