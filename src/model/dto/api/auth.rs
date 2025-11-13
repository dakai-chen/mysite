use serde::{Deserialize, Serialize};

use crate::model::bo::auth::{AdminAccessTokenBo, AdminLoginBo};

#[derive(Debug, Deserialize)]
pub struct AdminLoginDto {
    /// TOTP 动态口令
    pub totp_code: String,
}

impl<'a> Into<AdminLoginBo<'a>> for AdminLoginDto {
    fn into(self) -> AdminLoginBo<'a> {
        AdminLoginBo {
            totp_code: self.totp_code.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AdminAccessTokenDto {
    /// 访问令牌
    pub token: String,
    /// 访问令牌的过期时间
    pub expires_at: i64,
}

impl From<AdminAccessTokenBo> for AdminAccessTokenDto {
    fn from(value: AdminAccessTokenBo) -> Self {
        Self {
            token: value.token,
            expires_at: value.expires_at,
        }
    }
}
