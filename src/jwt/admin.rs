use serde::{Deserialize, Serialize};

use crate::util::jwt::JwtClaimsData;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminJwtData;

impl JwtClaimsData for AdminJwtData {
    fn kind() -> &'static str {
        "admin"
    }
}
