use serde::Serialize;

use crate::template::render::TemplateRenderData;

/// 首页
#[derive(Debug, Clone, Serialize)]
pub struct HomeVo;

impl TemplateRenderData for HomeVo {
    fn template_name() -> &'static str {
        "home.html"
    }
}
