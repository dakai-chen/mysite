use std::sync::LazyLock;

use totp_rs::TOTP;

use crate::error::{AppError, AppErrorMeta};
use crate::jwt::admin::AdminJwtData;
use crate::model::bo::auth::{AdminAccessTokenBo, AdminBo, AdminLoginBo};
use crate::util::jwt::JwtClaims;

static ADMIN_TOTP: LazyLock<TOTP> =
    LazyLock::new(|| TOTP::from_url(&crate::config::get().admin.totp_url).unwrap());

/// 管理员登录认证
pub async fn login(bo: &AdminLoginBo<'_>) -> Result<AdminAccessTokenBo, AppError> {
    if crate::config::get().admin.password != bo.password {
        return Err(AppErrorMeta::BadRequest.with_message("密码或口令错误"));
    }
    if ADMIN_TOTP.generate_current()? != bo.totp_code {
        return Err(AppErrorMeta::BadRequest.with_message("密码或口令错误"));
    }
    AdminAccessTokenBo::generate()
}

/// 验证管理员令牌
pub async fn validate_admin_token(token: &str) -> Result<AdminBo, AppError> {
    let claims =
        JwtClaims::<AdminJwtData>::decode(token, crate::config::get().jwt.secret.as_bytes())
            .map_err(|e| {
                AppErrorMeta::AdminAccessTokenInvalid
                    .with_source(e)
                    .with_context(format!("token: {token}"))
            })?;

    if !claims.is_effective() {
        return Err(AppErrorMeta::AdminAccessTokenNotEffective.into_error());
    }
    if claims.is_expired() {
        return Err(AppErrorMeta::AdminAccessTokenExpired.into_error());
    }

    Ok(AdminBo {
        expires_at: claims.exp,
    })
}
