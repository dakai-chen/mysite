use serde::Serialize;

use crate::template::render::TemplateRenderData;

/// 登录页面
#[derive(Debug, Clone, Serialize)]
pub struct AdminLoginVo;

impl TemplateRenderData for AdminLoginVo {
    fn template_name() -> &'static str {
        "auth/login.html"
    }
}

/// 登出页面
#[derive(Debug, Serialize)]
pub struct AdminLogoutVo;

impl TemplateRenderData for AdminLogoutVo {
    fn template_name() -> &'static str {
        "auth/logout.html"
    }
}
