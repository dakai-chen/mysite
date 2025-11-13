use std::pin::Pin;
use std::task::{Context, Poll};

use boluo::BoxError;
use boluo::body::{Body, Bytes, Frame, HttpBody, SizeHint};
use boluo::data::Extension;
use boluo::extract::{FromRequest, TypedHeader};
use boluo::handler::handler_fn;
use boluo::headers::ContentLength;
use boluo::middleware::Middleware;
use boluo::request::Request;
use boluo::response::Response;
use boluo::route::Route;
use boluo::route::{PathParams, RouteError, Router, any};
use boluo::service::{ArcService, Service, ServiceExt};

use crate::config::{BodyLimitConfig, BodyLimitRule};
use crate::error::AppErrorMeta;

#[derive(Debug, Clone)]
pub struct BodyLimit {
    helper: BodyLimitHelper,
}

impl BodyLimit {
    pub fn new(config: &BodyLimitConfig) -> anyhow::Result<Self> {
        Ok(Self {
            helper: BodyLimitHelper::from_config(config)?,
        })
    }
}

impl<S> Middleware<S> for BodyLimit
where
    S: Service<Request>,
{
    type Service = BodyLimitService<S>;

    fn transform(self, service: S) -> Self::Service {
        BodyLimitService {
            inner: service,
            helper: self.helper,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyLimitService<S> {
    inner: S,
    helper: BodyLimitHelper,
}

impl<S> Service<Request> for BodyLimitService<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;

    async fn call(&self, request: Request) -> Result<Self::Response, Self::Error> {
        let request = self.helper.apply_limit(request).await?;
        self.inner.call(request).await.map_err(Into::into)
    }
}

#[derive(Debug, Clone)]
struct BodyLimitHelper {
    service: ArcService<Request, Response, BoxError>,
}

impl BodyLimitHelper {
    pub fn from_config(config: &BodyLimitConfig) -> anyhow::Result<Self> {
        let router = Router::new();
        let router = config.rules.iter().try_fold(router, |router, rule| {
            Self::build_route_service(rule).map(|s| router.mount(s))
        })?;
        Ok(Self {
            service: router
                .with(Extension(BodyLimitValue(config.default_limit)))
                .boxed_arc(),
        })
    }

    fn build_route_service(
        rule: &BodyLimitRule,
    ) -> anyhow::Result<Route<impl Service<Request, Response = Response, Error = BoxError> + 'static>>
    {
        async fn return_request(request: Request) -> Result<(), RouteError> {
            Err(RouteError::not_found(request))
        }
        let service = any(handler_fn(return_request));
        let service = rule.method.iter().try_fold(service, |service, method| {
            method.parse().map(|m| service.add(m))
        })?;
        Ok(Route::new(&rule.path, service).with(Extension(BodyLimitValue(rule.limit))))
    }

    pub async fn apply_limit(&self, mut request: Request) -> Result<Request, BoxError> {
        let original_path_params = request.extensions_mut().insert(PathParams(vec![]));
        let error = self.service.call(request).await.unwrap_err();
        let error = error.downcast::<RouteError>()?;
        let mut request = error.into_request();
        if let Some(params) = original_path_params {
            request.extensions_mut().insert(params);
        }
        Self::check_and_apply_limit(request).await
    }

    async fn check_and_apply_limit(mut request: Request) -> Result<Request, BoxError> {
        let BodyLimitValue(Some(limit)) = request.extensions().get().copied().unwrap_or_default()
        else {
            return Ok(request);
        };
        if let Some(TypedHeader(ContentLength(content_length))) =
            Option::<TypedHeader<ContentLength>>::from_request(&mut request).await?
            && content_length > limit
        {
            return Err(AppErrorMeta::DataTooLarge { limit }.into_error().into());
        }
        Ok(request.map(|b| Body::new(Limited::new(b).with_limit(limit))))
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct BodyLimitValue(Option<u64>);

#[derive(Clone, Copy)]
struct LimitedState {
    /// 剩余可接收的字节数
    remaining: u64,
    /// 最大可接收的字节数
    max_allowed: u64,
}

pub struct Limited {
    inner: Body,
    limit: Option<LimitedState>,
}

impl Limited {
    /// 使用给定的 HttpBody 实现创建 Limited 实例。
    pub fn new<B>(body: B) -> Self
    where
        B: HttpBody<Data = Bytes> + Send + 'static,
        B::Error: Into<BoxError>,
    {
        Self {
            inner: Body::new(body),
            limit: None,
        }
    }

    /// 设置主体数据的大小限制并返回自身。
    ///
    /// 该方法功能与 [`Limited::set_limit`] 一致，详细说明请查看 [`Limited::set_limit`]。
    pub fn with_limit(mut self, limit: impl Into<Option<u64>>) -> Self {
        self.set_limit(limit);
        self
    }

    /// 设置主体数据的大小限制。
    ///
    /// 该限制用于控制允许接收的主体数据总字节数，超过此限制将返回 `BodyTooLarge` 错误。
    ///
    /// 限制仅对后续读取的数据流生效，已读取的数据不会计入限制计算
    pub fn set_limit(&mut self, limit: impl Into<Option<u64>>) {
        self.limit = limit.into().map(|max_allowed| LimitedState {
            remaining: max_allowed,
            max_allowed,
        });
    }

    /// 获取当前设置的主体数据大小限制。
    pub fn limit(&self) -> Option<u64> {
        self.limit.map(|limit| limit.max_allowed)
    }

    pub fn into_inner(self) -> Body {
        self.inner
    }
}

impl HttpBody for Limited {
    type Data = Bytes;
    type Error = BoxError;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let Limited { inner, limit } = self.get_mut();
        if let Some(limit) = limit {
            match Pin::new(inner).poll_frame(cx) {
                Poll::Ready(Some(Ok(frame))) => {
                    if let Some(data) = frame.data_ref() {
                        if let Ok(data_remaining) = u64::try_from(data.len()) {
                            if data_remaining > limit.remaining {
                                limit.remaining = 0;
                                Poll::Ready(Some(Err(AppErrorMeta::DataTooLarge {
                                    limit: limit.max_allowed,
                                }
                                .into_error()
                                .into())))
                            } else {
                                limit.remaining -= data_remaining;
                                Poll::Ready(Some(Ok(frame)))
                            }
                        } else {
                            limit.remaining = 0;
                            Poll::Ready(Some(Err(AppErrorMeta::DataTooLarge {
                                limit: limit.max_allowed,
                            }
                            .into_error()
                            .into())))
                        }
                    } else {
                        Poll::Ready(Some(Ok(frame)))
                    }
                }
                res => res,
            }
        } else {
            Pin::new(inner).poll_frame(cx)
        }
    }

    fn size_hint(&self) -> SizeHint {
        let mut hint = self.inner.size_hint();
        if let Some(limit) = self.limit {
            if hint.lower() >= limit.remaining {
                hint.set_exact(limit.remaining);
            } else if let Some(upper) = hint.upper() {
                hint.set_upper(limit.remaining.min(upper));
            } else {
                hint.set_upper(limit.remaining);
            }
        }
        hint
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }
}
