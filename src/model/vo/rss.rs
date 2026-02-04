use serde::Serialize;

use crate::model::bo::article::ArticleListBo;
use crate::model::vo::article::ArticleListItemVo;
use crate::template::render::TemplateRenderData;

/// RSS 页面
#[derive(Debug, Clone, Serialize)]
pub struct RssVo {
    /// 文章列表数据
    pub items: Vec<ArticleListItemVo>,
}

impl From<ArticleListBo> for RssVo {
    fn from(value: ArticleListBo) -> Self {
        Self {
            items: value
                .data
                .items
                .into_iter()
                .map(ArticleListItemVo::from)
                .collect(),
        }
    }
}

impl TemplateRenderData for RssVo {
    fn template_name() -> &'static str {
        "rss.xml"
    }
}
