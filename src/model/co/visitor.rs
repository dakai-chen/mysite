use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::storage::cache::CacheData;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisitorCo {
    /// 访客ID
    pub visitor_id: String,
}

impl CacheData for VisitorCo {
    fn kind() -> &'static str {
        "visitor"
    }

    fn generate_id(&self) -> Cow<'_, str> {
        self.visitor_id.as_str().into()
    }
}
