use std::ops::{Deref, DerefMut};

use boluo::extract::{FromRequest, OptionalFromRequest, TypedHeader};
use boluo::headers::Authorization;
use boluo::headers::authorization::Bearer;
use boluo::http::header::SET_COOKIE;
use boluo::request::Request;
use boluo::response::{IntoResponseParts, ResponseParts};
use boluo_extra::cookie::{Cookie, CookieJar, SameSite};
use time::{Duration, OffsetDateTime};

use crate::error::{AppError, AppErrorMeta};
use crate::model::bo::auth::{AdminAccessTokenBo, AdminBo};

const COOKIE_KEY: &str = "admin";

#[derive(Debug)]
pub struct Admin(pub AdminBo);

impl Deref for Admin {
    type Target = AdminBo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Admin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<AdminBo> for Admin {
    fn into(self) -> AdminBo {
        self.0
    }
}

impl OptionalFromRequest for Admin {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Option<Self>, Self::Error> {
        if let Some(admin) = Option::<AdminFromHeader>::from_request(request).await? {
            return Ok(Some(Self(admin.into())));
        }
        if let Some(admin) = Option::<AdminFromCookie>::from_request(request).await? {
            return Ok(Some(Self(admin.into())));
        }
        Ok(None)
    }
}

impl FromRequest for Admin {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        Option::<Admin>::from_request(request)
            .await?
            .ok_or_else(|| AppErrorMeta::AdminAccessTokenMissing.into_error())
    }
}

#[derive(Debug)]
pub struct AdminFromHeader(pub AdminBo);

impl Deref for AdminFromHeader {
    type Target = AdminBo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdminFromHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<AdminBo> for AdminFromHeader {
    fn into(self) -> AdminBo {
        self.0
    }
}

impl OptionalFromRequest for AdminFromHeader {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Option<Self>, Self::Error> {
        let Some(TypedHeader(token)) =
            Option::<TypedHeader<Authorization<Bearer>>>::from_request(request)
                .await
                .map_err(|e| AppErrorMeta::AdminAccessTokenInvalid.with_source(e))?
        else {
            return Ok(None);
        };

        let admin = crate::service::auth::validate_admin_token(token.token()).await?;

        Ok(Some(AdminFromHeader(admin)))
    }
}

impl FromRequest for AdminFromHeader {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        Option::<AdminFromHeader>::from_request(request)
            .await?
            .ok_or_else(|| AppErrorMeta::AdminAccessTokenMissing.into_error())
    }
}

#[derive(Debug)]
pub struct AdminFromCookie(pub AdminBo);

impl Deref for AdminFromCookie {
    type Target = AdminBo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdminFromCookie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<AdminBo> for AdminFromCookie {
    fn into(self) -> AdminBo {
        self.0
    }
}

impl OptionalFromRequest for AdminFromCookie {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Option<Self>, Self::Error> {
        let jar = CookieJar::from_request(request).await.map_err(|e| {
            AppErrorMeta::BadRequest
                .with_message("Cookie 解析错误")
                .with_source(e)
        })?;

        let Some(cookie) = jar.get(COOKIE_KEY) else {
            return Ok(None);
        };

        let admin = crate::service::auth::validate_admin_token(cookie.value()).await?;

        Ok(Some(AdminFromCookie(admin)))
    }
}

impl FromRequest for AdminFromCookie {
    type Error = AppError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        Option::<AdminFromCookie>::from_request(request)
            .await?
            .ok_or_else(|| AppErrorMeta::AdminAccessTokenMissing.into_error())
    }
}

#[derive(Debug, Clone)]
pub struct AdminWriteCookie {
    /// 访问令牌
    pub token: String,
    /// 访问令牌的过期时间
    pub expires_at: i64,
}

impl AdminWriteCookie {
    fn build_cookie(&self) -> anyhow::Result<Cookie<'_>> {
        let mut cookie = Cookie::new(COOKIE_KEY, &self.token);
        cookie.set_expires(self.cookie_expires_at()?);
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        if crate::config::get().security.cookie_secure {
            cookie.set_secure(true);
        }
        Ok(cookie)
    }

    fn cookie_expires_at(&self) -> anyhow::Result<OffsetDateTime> {
        Ok(OffsetDateTime::from_unix_timestamp(self.expires_at)?)
    }
}

impl From<AdminAccessTokenBo> for AdminWriteCookie {
    fn from(value: AdminAccessTokenBo) -> Self {
        Self {
            token: value.token,
            expires_at: value.expires_at,
        }
    }
}

impl IntoResponseParts for AdminWriteCookie {
    type Error = AppError;

    fn into_response_parts(self, mut parts: ResponseParts) -> Result<ResponseParts, Self::Error> {
        if let Ok(value) = self.build_cookie()?.encoded().to_string().parse() {
            parts.headers.append(SET_COOKIE, value);
        }
        Ok(parts)
    }
}

#[derive(Debug, Clone)]
pub struct AdminCleanCookie;

impl AdminCleanCookie {
    fn build_cookie(&self) -> anyhow::Result<Cookie<'_>> {
        let mut cookie = Cookie::from(COOKIE_KEY);
        cookie.set_max_age(Duration::seconds(0));
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        if crate::config::get().security.cookie_secure {
            cookie.set_secure(true);
        }
        Ok(cookie)
    }
}

impl IntoResponseParts for AdminCleanCookie {
    type Error = AppError;

    fn into_response_parts(self, mut parts: ResponseParts) -> Result<ResponseParts, Self::Error> {
        if let Ok(value) = self.build_cookie()?.encoded().to_string().parse() {
            parts.headers.append(SET_COOKIE, value);
        }
        Ok(parts)
    }
}
