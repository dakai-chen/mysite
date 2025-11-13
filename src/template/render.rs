use serde::Serialize;

use crate::config::AppConfig;
use crate::model::bo::auth::AdminBo;

pub trait TemplateRenderData: Serialize {
    fn template_name() -> &'static str;
}

#[derive(Debug, Serialize)]
pub struct PageContext<T> {
    pub admin: Option<AdminBo>,
    pub config: &'static AppConfig,
    pub context: T,
}

impl<T> PageContext<T> {
    pub fn new(context: T) -> Self {
        Self {
            admin: None,
            config: crate::config::get(),
            context,
        }
    }

    pub fn admin<U>(mut self, admin: U) -> Self
    where
        U: Into<Option<AdminBo>>,
    {
        self.admin = admin.into();
        self
    }
}
