mod format;
mod markdown;

use tera::Tera;

use crate::config::ThemeConfig;
use crate::template::helper::markdown::MarkdownToHtml;

pub fn register_helper(tera: &mut Tera, config: &ThemeConfig) -> anyhow::Result<()> {
    tera.register_filter("markdown_to_html", MarkdownToHtml::from_config(config)?);
    tera.register_filter("human_number", format::human_number);
    tera.register_filter("human_size", format::human_size);
    Ok(())
}
