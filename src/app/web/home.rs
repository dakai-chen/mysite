use std::sync::Arc;

use boluo::BoxError;
use boluo::data::Extension;
use boluo::response::{Html, IntoResponse};

use crate::context::auth::AdminFromCookie;
use crate::model::vo::home::HomeVo;
use crate::state::AppState;
use crate::template::render::PageContext;

#[boluo::route("/", method = ["GET"])]
pub async fn home(
    admin: Option<AdminFromCookie>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, BoxError> {
    let context = PageContext::new(HomeVo).admin(admin.map(Into::into));
    Ok(Html(state.template.typed_render(&context)))
}
