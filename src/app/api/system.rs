use std::sync::Arc;
use std::time::Duration;

use boluo::BoxError;
use boluo::data::{Extension, Json};
use boluo::response::IntoResponse;

use crate::context::auth::Admin;
use crate::model::dto::api::system::{InfoDto, SetLogLevelDto, SetShutdownTimeoutDto};
use crate::state::AppState;
use crate::validator::Validation;

#[boluo::route("/system/set_log_level", method = "POST")]
pub async fn set_log_level(
    _: Admin,
    Json(params): Json<SetLogLevelDto>,
) -> Result<impl IntoResponse, BoxError> {
    params.validate(&())?;
    crate::logger::set_level(&params.level)?;
    Ok(crate::response::ok(()))
}

#[boluo::route("/system/get_log_level", method = "POST")]
pub async fn get_log_level(_: Admin) -> Result<impl IntoResponse, BoxError> {
    Ok(crate::response::ok(
        serde_json::json!({ "level": crate::logger::get_level()? }),
    ))
}

#[boluo::route("/system/set_shutdown_timeout", method = "POST")]
pub async fn set_shutdown_timeout(
    _: Admin,
    Json(params): Json<SetShutdownTimeoutDto>,
) -> Result<impl IntoResponse, BoxError> {
    params.validate(&())?;
    crate::shutdown::set_timeout(Duration::from_secs(params.timeout));
    Ok(crate::response::ok(()))
}

#[boluo::route("/system/get_shutdown_timeout", method = "POST")]
pub async fn get_shutdown_timeout(_: Admin) -> impl IntoResponse {
    crate::response::ok(
        serde_json::json!({ "timeout": crate::shutdown::timeout().map(|v| v.as_secs()) }),
    )
}

#[boluo::route("/system/info", method = "POST")]
pub async fn info(
    _: Admin,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, BoxError> {
    let info = crate::service::system::info(state).await?;
    Ok(crate::response::ok(InfoDto::from(info)))
}
