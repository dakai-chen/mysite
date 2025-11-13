use crate::error::AppError;
use crate::model::bo::visitor::VisitorBo;

pub async fn keep_or_create_visitor(visitor_id: &str) -> Result<VisitorBo, AppError> {
    if VisitorBo::keep(visitor_id).await? {
        let Some(visitor) = VisitorBo::from_cache(visitor_id).await? else {
            return Ok(VisitorBo::create_and_cache().await?);
        };
        return Ok(visitor);
    }
    Ok(VisitorBo::create_and_cache().await?)
}

pub async fn create_visitor() -> Result<VisitorBo, AppError> {
    Ok(VisitorBo::create_and_cache().await?)
}
