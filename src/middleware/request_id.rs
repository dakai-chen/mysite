use std::str::FromStr;

use boluo::BoxError;
use boluo::http::{HeaderName, HeaderValue};
use boluo::middleware::Middleware;
use boluo::request::Request;
use boluo::response::{IntoResponse, Response};
use boluo::service::Service;
use tracing::Span;

use crate::error::AppErrorMeta;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestId(String);

impl RequestId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdMiddleware {
    header_name: HeaderName,
}

impl RequestIdMiddleware {
    pub fn with_header_name(header_name: &str) -> Self {
        Self {
            header_name: HeaderName::from_str(header_name).unwrap(),
        }
    }
}

impl<S> Middleware<S> for RequestIdMiddleware {
    type Service = RequestIdService<S>;

    fn transform(self, service: S) -> Self::Service {
        RequestIdService {
            inner: service,
            header_name: self.header_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdService<S> {
    inner: S,
    header_name: HeaderName,
}

impl<S> Service<Request> for RequestIdService<S>
where
    S: Service<Request>,
    S::Response: IntoResponse,
    S::Error: Into<BoxError>,
{
    type Response = Response;
    type Error = BoxError;

    async fn call(&self, mut request: Request) -> Result<Self::Response, Self::Error> {
        let request_id = request_id_from_request(&mut request, &self.header_name)?;
        request.extensions_mut().insert(request_id.clone());

        Span::current().record("request_id", request_id.as_str());

        let mut response = self.inner.call(request).await.into_response()?;
        if let Ok(val) = HeaderValue::from_str(request_id.as_str()) {
            response.headers_mut().insert(&self.header_name, val);
        }
        Ok(response)
    }
}

fn request_id_from_request(
    request: &mut Request,
    header_name: &HeaderName,
) -> Result<RequestId, BoxError> {
    Ok(match request.headers().get(header_name) {
        Some(header_value) => RequestId::new(header_value.to_str().map_err(|_| {
            AppErrorMeta::BadRequest
                .with_message(format!("请求头 `{header_name}` 包含禁止字符或无效 UTF-8",))
        })?),
        None => RequestId::new(crate::util::uuid::v4()),
    })
}
