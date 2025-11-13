use std::str::FromStr;

use mime::Mime;

use crate::model::dto::api::resource::{DownloadResourceDto, RemoveResourceDto, UploadResourceDto};
use crate::validator::{Validation, ValidationError};

impl Validation<()> for UploadResourceDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        if self.meta.size == 0 {
            return Err(ValidationError::validation("禁止上传空文件"));
        }
        if self.meta.name.trim().is_empty() {
            return Err(ValidationError::validation("文件名不能为空或仅包含空格"));
        }
        if self.meta.name.len() > 255 {
            return Err(ValidationError::validation("文件名过长，最大支持 255 字节"));
        }
        if !crate::util::path::is_safe(&self.meta.name) {
            return Err(ValidationError::validation("文件名包含禁止字符"));
        }
        if Mime::from_str(&self.meta.mime_type).is_err() {
            return Err(ValidationError::validation("无效的 MIME 类型格式"));
        }
        Ok(())
    }
}

impl Validation<()> for DownloadResourceDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        if self.resource_id.is_empty() {
            return Err(ValidationError::validation("资源ID不能为空"));
        }
        Ok(())
    }
}

impl Validation<()> for RemoveResourceDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        if self.resource_id.is_empty() {
            return Err(ValidationError::validation("资源ID不能为空"));
        }
        Ok(())
    }
}
