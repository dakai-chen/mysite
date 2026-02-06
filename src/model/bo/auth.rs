use std::borrow::Cow;

use serde::Serialize;

use crate::error::AppError;
use crate::jwt::admin::AdminJwtData;
use crate::util::jwt::JwtClaimsData;

#[derive(Debug, Serialize)]
pub struct AdminBo {
    /// 过期时间
    pub expires_at: i64,
}

#[derive(Debug)]
pub struct AdminLoginBo<'a> {
    /// 登录密码
    pub password: Cow<'a, str>,
    /// TOTP 动态口令
    pub totp_code: Cow<'a, str>,
}

#[derive(Debug)]
pub struct AdminAccessTokenBo {
    /// 访问令牌
    pub token: String,
    /// 访问令牌的过期时间
    pub expires_at: i64,
}

impl AdminAccessTokenBo {
    pub fn generate() -> Result<Self, AppError> {
        let claims = AdminJwtData.with_ttl(crate::config::get().admin.session_ttl);
        let token = claims.encode(crate::config::get().jwt.secret.as_bytes())?;
        Ok(Self {
            token,
            expires_at: claims.exp,
        })
    }
}
