pub mod helper;
pub mod render;

use serde::Serialize;
use tera::{Context, Tera};

use crate::config::ThemeConfig;
use crate::template::render::{PageContext, TemplateRenderData};

pub fn build_template(config: &ThemeConfig) -> anyhow::Result<TemplateEngine> {
    let mut tera = Tera::new(
        &crate::util::path::root(&config.current().templates_dir)
            .join("**/*.html")
            .into_string(),
    )?;

    helper::register_helper(&mut tera, config)?;

    Ok(TemplateEngine { tera })
}

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn raw_render(&self, template_name: &str) -> anyhow::Result<String> {
        self.tera
            .render(template_name, &Context::default())
            .map_err(Into::into)
    }

    pub fn raw_render_with<T>(&self, template_name: &str, context: &T) -> anyhow::Result<String>
    where
        T: Serialize,
    {
        self.tera
            .render(template_name, &Context::from_serialize(context)?)
            .map_err(Into::into)
    }

    pub fn render<T>(&self, data: &PageContext<T>) -> anyhow::Result<String>
    where
        T: TemplateRenderData,
    {
        self.raw_render_with(T::template_name(), data)
    }
}
