use serde::Deserialize;

use crate::model::bo::auth::AdminLoginBo;

#[derive(Debug, Deserialize)]
pub struct AdminLoginSubmitDto {
    /// 登录密码
    pub password: String,
    /// TOTP 动态口令
    pub totp_code: String,
}

impl<'a> Into<AdminLoginBo<'a>> for AdminLoginSubmitDto {
    fn into(self) -> AdminLoginBo<'a> {
        AdminLoginBo {
            password: self.password.into(),
            totp_code: self.totp_code.into(),
        }
    }
}
