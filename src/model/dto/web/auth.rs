use serde::Deserialize;

use crate::model::bo::auth::AdminLoginBo;

#[derive(Debug, Deserialize)]
pub struct AdminLoginSubmitDto {
    /// TOTP 动态口令
    pub totp_code: String,
}

impl<'a> Into<AdminLoginBo<'a>> for AdminLoginSubmitDto {
    fn into(self) -> AdminLoginBo<'a> {
        AdminLoginBo {
            totp_code: self.totp_code.into(),
        }
    }
}
