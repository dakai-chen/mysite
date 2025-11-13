use crate::model::dto::api::system::{SetLogLevelDto, SetShutdownTimeoutDto};
use crate::validator::{Validation, ValidationError};

impl Validation<()> for SetLogLevelDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        if self.level.is_empty() {
            return Err(ValidationError::validation("日志等级不能为空"));
        }
        Ok(())
    }
}

impl Validation<()> for SetShutdownTimeoutDto {
    fn validate(&self, _context: &()) -> Result<(), ValidationError> {
        Ok(())
    }
}
