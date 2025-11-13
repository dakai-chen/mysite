use std::marker::PhantomData;

use boluo::BoxError;
use boluo::extract::FromRequest;
use boluo::middleware::Middleware;
use boluo::request::Request;
use boluo::service::Service;

#[derive(Debug, Clone, Copy)]
pub struct OrElseWith<F, T> {
    handle: F,
    _marker: PhantomData<fn(T) -> T>,
}

impl<F, T> OrElseWith<F, T> {
    pub fn new(handle: F) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }
}

impl<S, F, T> Middleware<S> for OrElseWith<F, T> {
    type Service = OrElseWithService<S, F, T>;

    fn transform(self, service: S) -> Self::Service {
        OrElseWithService {
            inner: service,
            handle: self.handle,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrElseWithService<S, F, T> {
    inner: S,
    handle: F,
    _marker: PhantomData<fn(T) -> T>,
}

impl<S, F, T, Fut, Err> Service<Request> for OrElseWithService<S, F, T>
where
    S: Service<Request>,
    T: FromRequest + Send + Sync,
    T::Error: Into<BoxError>,
    F: Fn(S::Error, T) -> Fut + Send + Sync,
    Fut: Future<Output = Result<S::Response, Err>> + Send,
    Err: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;

    async fn call(&self, mut request: Request) -> Result<Self::Response, Self::Error> {
        let t = T::from_request(&mut request).await.map_err(Into::into)?;
        let err = match self.inner.call(request).await {
            Ok(res) => return Ok(res),
            Err(err) => err,
        };
        (self.handle)(err, t).await.map_err(Into::into)
    }
}
