use std::sync::Arc;

use boluo::BoxError;
use boluo::data::Extension;
use boluo::extract::FromRequest;
use boluo::request::Request;

use crate::state::AppState;

pub struct DbPoolConnection(pub crate::storage::db::DbPoolConn);

impl FromRequest for DbPoolConnection {
    type Error = BoxError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Error> {
        let Extension(state) = Extension::<Arc<AppState>>::from_request(request).await?;
        Ok(DbPoolConnection(state.db.acquire().await?))
    }
}
