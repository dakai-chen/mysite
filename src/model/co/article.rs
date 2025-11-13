use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::storage::cache::CacheData;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisitorArticleAccessPermitCo<'a> {
    /// 访客ID
    pub visitor_id: Cow<'a, str>,
    /// 文章ID
    pub article_id: Cow<'a, str>,
}

impl CacheData for VisitorArticleAccessPermitCo<'_> {
    fn kind() -> &'static str {
        "visitor_article_access_permit"
    }

    fn generate_id(&self) -> Cow<'_, str> {
        format!("{}:{}", self.visitor_id, self.article_id).into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisitorArticleAccessRecordCo<'a> {
    /// 访客ID
    pub visitor_id: Cow<'a, str>,
    /// 文章ID
    pub article_id: Cow<'a, str>,
}

impl CacheData for VisitorArticleAccessRecordCo<'_> {
    fn kind() -> &'static str {
        "visitor_article_access_record"
    }

    fn generate_id(&self) -> Cow<'_, str> {
        format!("{}:{}", self.visitor_id, self.article_id).into()
    }
}
