use serde::{Deserialize, Serialize};

use crate::util::jwt::JwtClaimsData;

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleAccessJwtData {
    /// 文章ID
    pub article_id: String,
}

impl JwtClaimsData for ArticleAccessJwtData {
    fn kind() -> &'static str {
        "article-access"
    }
}
