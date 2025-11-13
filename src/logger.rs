use std::str::FromStr;
use std::sync::OnceLock;

use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::reload::{self, Handle};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::config::LoggerConfig;
use crate::util::time::UnixTimestampSecs;

static ENV_FILTER_HANDLE: OnceLock<Handle<EnvFilter, Registry>> = OnceLock::new();

pub fn set_level(level: &str) -> anyhow::Result<()> {
    let new = EnvFilter::from_str(level)?;
    ENV_FILTER_HANDLE
        .get()
        .ok_or_else(|| anyhow::anyhow!("全局日志未初始化"))?
        .reload(new)
        .map_err(From::from)
}

pub fn get_level() -> anyhow::Result<String> {
    ENV_FILTER_HANDLE
        .get()
        .ok_or_else(|| anyhow::anyhow!("全局日志未初始化"))?
        .with_current(|filter| filter.to_string())
        .map_err(From::from)
}

/// 初始化日志
pub fn init(config: &LoggerConfig) -> anyhow::Result<Option<WorkerGuard>> {
    let (filter, handle) = reload::Layer::new(EnvFilter::from_str(&config.level)?);
    ENV_FILTER_HANDLE.set(handle).unwrap();

    let registry = tracing_subscriber::registry().with(filter);
    let fmt = tracing_subscriber::fmt::layer().with_timer(CustomTime);

    if !config.enable_file_output {
        registry.with(fmt).try_init()?;
        return Ok(None);
    }

    let (writer, guard) = build_rolling_file_appender(config)?;

    registry
        .with(fmt.with_writer(writer).with_ansi(false))
        .try_init()?;

    Ok(Some(guard))
}

fn build_rolling_file_appender(
    config: &LoggerConfig,
) -> anyhow::Result<(NonBlocking, WorkerGuard)> {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(&config.file_prefix)
        .max_log_files(config.max_keep_files)
        .build(&config.file_dir)?;
    Ok(tracing_appender::non_blocking(appender))
}

struct CustomTime;

impl FormatTime for CustomTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        UnixTimestampSecs::now()
            .into_str_iso8601()
            .map_err(|_| std::fmt::Error)
            .and_then(|t| write!(w, "{t}"))
    }
}
