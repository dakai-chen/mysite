use boluo::BoxError;
use boluo::data::Json;
use boluo::response::IntoResponse;

use crate::model::dto::api::auth::AdminAccessTokenDto;
use crate::model::dto::api::auth::AdminLoginDto;
use crate::validator::Validation;

#[boluo::route("/auth/login", method = "POST")]
pub async fn login(Json(params): Json<AdminLoginDto>) -> Result<impl IntoResponse, BoxError> {
    params.validate(&())?;
    let token = crate::service::auth::login(&params.into()).await?;
    Ok(crate::response::ok(AdminAccessTokenDto::from(token)))
}
