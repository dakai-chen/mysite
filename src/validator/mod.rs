mod bo;
mod common;
mod dto;

pub trait Validation<C> {
    fn validate(&self, context: &C) -> Result<(), ValidationError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    /// 业务验证错误
    #[error("数据验证失败：{0}")]
    Validation(String),
    /// 内部系统错误
    #[error("系统内部错误：{0}")]
    Internal(String),
}

impl ValidationError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn internal<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::Internal(error.to_string())
    }
}
