use std::sync::Mutex;
use std::time::Duration;

#[cfg(unix)]
pub async fn signal() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut stream1 = signal(SignalKind::hangup()).unwrap();
    let mut stream2 = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = stream1.recv() => {}
        _ = stream2.recv() => {}
    }
}

#[cfg(not(unix))]
pub async fn signal() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
    }
}

static TIMEOUT: Mutex<Option<Duration>> = Mutex::new(None);

pub fn set_timeout(value: impl Into<Option<Duration>>) {
    *TIMEOUT.lock().unwrap_or_else(|e| e.into_inner()) = value.into();
}

pub fn timeout() -> Option<Duration> {
    *TIMEOUT.lock().unwrap_or_else(|e| e.into_inner())
}

pub async fn graceful() -> Option<Duration> {
    signal().await;
    let timeout = timeout();
    if let Some(timeout) = timeout {
        tracing::info!("HTTP 服务开始优雅关闭，等待活跃请求处理完成（超时时间：{timeout:?}）");
    } else {
        tracing::info!("HTTP 服务开始优雅关闭，等待活跃请求处理完成");
    }
    timeout
}
