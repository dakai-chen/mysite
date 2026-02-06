use crate::model::dto::web::auth::AdminLoginSubmitDto;
use crate::validator::{Validation, ValidationError};

impl Validation<()> for AdminLoginSubmitDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        if self.password.is_empty() {
            return Err(ValidationError::validation("密码不能为空"));
        }
        if self.totp_code.is_empty() {
            return Err(ValidationError::validation("口令不能为空"));
        }
        if self.totp_code.len() != 6 || !self.totp_code.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::validation("口令必须为6位数字"));
        }
        Ok(())
    }
}
