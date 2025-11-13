use std::borrow::Cow;

use serde::Serialize;

use crate::{error::AppError, template::render::TemplateRenderData};

#[derive(Debug, Clone, Serialize)]
pub struct Err404Vo<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> Err404Vo<'a> {
    pub fn from(error: &'a AppError) -> Self {
        Self {
            message: error.message().into(),
        }
    }
}

impl TemplateRenderData for Err404Vo<'_> {
    fn template_name() -> &'static str {
        "error/404.html"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Err405Vo<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> Err405Vo<'a> {
    pub fn from(error: &'a AppError) -> Self {
        Self {
            message: error.message().into(),
        }
    }
}

impl TemplateRenderData for Err405Vo<'_> {
    fn template_name() -> &'static str {
        "error/405.html"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrOtherVo<'a> {
    pub status_code: u16,
    pub code: Cow<'a, str>,
    pub message: Cow<'a, str>,
}

impl<'a> ErrOtherVo<'a> {
    pub fn from(error: &'a AppError) -> Self {
        Self {
            status_code: error.meta().status_code().as_u16(),
            code: error.meta().code().into(),
            message: error.message().into(),
        }
    }
}

impl TemplateRenderData for ErrOtherVo<'_> {
    fn template_name() -> &'static str {
        "error/other.html"
    }
}
