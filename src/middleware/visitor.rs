use boluo::BoxError;
use boluo::extract::FromRequest;
use boluo::middleware::Middleware;
use boluo::request::Request;
use boluo::response::{IntoResponse, Response};
use boluo::service::Service;

use crate::context::visitor::VisitorId;

#[derive(Debug, Clone, Copy)]
pub struct VisitorMiddleware;

impl<S> Middleware<S> for VisitorMiddleware {
    type Service = VisitorService<S>;

    fn transform(self, service: S) -> Self::Service {
        VisitorService { service }
    }
}

impl VisitorMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VisitorService<S> {
    service: S,
}

impl<S> Service<Request> for VisitorService<S>
where
    S: Service<Request>,
    S::Response: IntoResponse,
    S::Error: Into<BoxError>,
{
    type Response = Response;
    type Error = BoxError;

    async fn call(&self, mut request: Request) -> Result<Self::Response, Self::Error> {
        let visitor = VisitorId::from_request(&mut request).await?;
        let response = match self.service.call(request).await.into_response() {
            Ok(response) => response,
            Err(error) => crate::response::error(error),
        };
        (visitor, response).into_response()
    }
}
