use boluo::BoxError;
use boluo::http::StatusCode;
use boluo::response::{IntoResponse, Json, Response};
use serde::Serialize;

use crate::error::AppError;

pub fn ok<T: Serialize>(data: T) -> impl IntoResponse<Error = BoxError> {
    Json(ResponseFormat::new("OK").data(data)).into_response()
}

pub fn error(error: BoxError) -> Response {
    let app_error = crate::error::into_app_error(error);
    tracing::error!("{app_error}");
    match app_error_to_response(app_error).into_response() {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("AppError to Response failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response_always()
        }
    }
}

fn app_error_to_response(error: AppError) -> impl IntoResponse<Error = BoxError> {
    let msg = error.message().to_owned();
    let res = ResponseFormat::<()>::new(error.meta().code()).message(msg);
    (error.meta().status_code(), Json(res))
}

#[derive(Debug, Serialize)]
struct ResponseFormat<T> {
    /// 错误码
    code: &'static str,
    /// 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    /// 消息
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ResponseFormat<T> {
    fn new(code: &'static str) -> Self {
        ResponseFormat {
            code,
            data: None,
            message: None,
        }
    }

    fn code(mut self, code: &'static str) -> Self {
        self.code = code;
        self
    }

    fn data(mut self, data: impl Into<Option<T>>) -> Self {
        self.data = data.into();
        self
    }

    fn message(mut self, message: impl Into<Option<String>>) -> Self {
        self.message = message.into();
        self
    }
}
