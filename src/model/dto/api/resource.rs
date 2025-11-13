use std::pin::Pin;

use boluo::BoxError;
use boluo::body::{Body, Bytes};
use boluo::extract::FromRequest;
use boluo::http::HeaderMap;
use boluo::request::Request;
use futures_util::Stream;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::error::{AppError, AppErrorMeta};
use crate::model::bo::resource::{
    FindResourceBo, RemoveResourceBo, ResourceBo, UploadResourceBo, UploadResourceMetaBo,
};

/// 上传资源文件元数据
#[derive(Debug, Clone)]
pub struct UploadResourceMetaDto {
    /// 文件名
    pub name: String,
    /// 文件大小
    pub size: u64,
    /// 文件类型
    pub mime_type: String,
    /// 文件哈希
    pub sha256: String,
}

impl Into<UploadResourceMetaBo> for UploadResourceMetaDto {
    fn into(self) -> UploadResourceMetaBo {
        UploadResourceMetaBo {
            name: self.name,
            size: self.size,
            mime_type: self.mime_type,
            sha256: self.sha256,
        }
    }
}

/// 上传资源文件
pub struct UploadResourceDto {
    /// 文件元数据
    pub meta: UploadResourceMetaDto,
    /// 文件数据
    pub data: Pin<Box<dyn Stream<Item = Result<Bytes, BoxError>> + Send>>,
}

impl UploadResourceDto {
    pub fn from_http(headers: &HeaderMap, data: Body) -> Result<Self, AppError> {
        let name = crate::util::http::typed_header::<String>(headers, "x-file-name")
            .map_err(|e| AppErrorMeta::BadRequest.with_message(e))?;
        let name = urlencoding::decode(&name)
            .map_err(|e| {
                AppErrorMeta::BadRequest.with_message(format!(
                    "文件名解码失败：{e}，请确保文件名使用 UTF-8 编码后再进行 urlencode 编码"
                ))
            })?
            .to_string();
        let size = crate::util::http::typed_header::<u64>(headers, "x-file-size")
            .map_err(|e| AppErrorMeta::BadRequest.with_message(e))?;
        let mime_type = crate::util::http::typed_header::<String>(headers, "x-file-mime-type")
            .map_err(|e| AppErrorMeta::BadRequest.with_message(e))?;
        let sha256 = crate::util::http::typed_header::<String>(headers, "x-file-sha256")
            .map_err(|e| AppErrorMeta::BadRequest.with_message(e))?;
        Ok(Self {
            meta: UploadResourceMetaDto {
                name,
                size,
                mime_type,
                sha256,
            },
            data: Box::pin(data.into_data_stream()),
        })
    }
}

impl FromRequest for UploadResourceDto {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        let body = std::mem::take(request.body_mut());
        Self::from_http(request.headers(), body)
    }
}

impl Into<UploadResourceBo> for UploadResourceDto {
    fn into(self) -> UploadResourceBo {
        UploadResourceBo {
            meta: self.meta.into(),
            data: self.data,
        }
    }
}

/// 资源文件
#[derive(Debug, Clone)]
pub struct ResourceDto {
    /// 资源ID
    pub resource_id: String,
    /// 文件名
    pub name: String,
    /// 文件扩展名
    pub extension: String,
    /// 文件大小
    pub size: u64,
    /// 文件类型
    pub mime_type: String,
    /// 文件哈希
    pub sha256: String,
    /// 创建时间
    pub created_at: i64,
}

impl From<ResourceBo> for ResourceDto {
    fn from(value: ResourceBo) -> Self {
        Self {
            resource_id: value.resource_id,
            name: value.name,
            extension: value.extension,
            size: value.size,
            mime_type: value.mime_type,
            sha256: value.sha256,
            created_at: value.created_at,
        }
    }
}

impl Serialize for ResourceDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceDto", 8)?;

        state.serialize_field("resource_id", &self.resource_id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("mime_type", &self.mime_type)?;
        state.serialize_field("sha256", &self.sha256)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("url", &format!("/resources/{}", self.resource_id))?;

        state.end()
    }
}

/// 下载资源文件
#[derive(Debug, Clone, Deserialize)]
pub struct DownloadResourceDto {
    /// 资源文件ID
    pub resource_id: String,
}

impl<'a> Into<FindResourceBo<'a>> for DownloadResourceDto {
    fn into(self) -> FindResourceBo<'a> {
        FindResourceBo {
            resource_id: self.resource_id.into(),
        }
    }
}

/// 删除资源文件
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveResourceDto {
    /// 资源文件ID
    pub resource_id: String,
}

impl<'a> Into<RemoveResourceBo<'a>> for RemoveResourceDto {
    fn into(self) -> RemoveResourceBo<'a> {
        RemoveResourceBo {
            resource_id: self.resource_id.into(),
        }
    }
}
