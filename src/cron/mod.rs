mod core;
mod task;

use std::sync::{Arc, OnceLock};

use tokio_cron_scheduler::JobScheduler;

use crate::cron::core::CronTaskCollector;
use crate::state::AppState;

static SCHEDULER: OnceLock<JobScheduler> = OnceLock::new();

pub fn get() -> &'static JobScheduler {
    SCHEDULER.get().expect("定时任务未初始化")
}

pub async fn start() -> anyhow::Result<()> {
    tracing::info!("开始启动定时任务");
    get().start().await?;
    tracing::info!("定时任务启动完成");
    Ok(())
}

pub async fn shutdown() -> anyhow::Result<()> {
    tracing::info!("开始关闭定时任务");
    get().clone().shutdown().await?;
    tracing::info!("定时任务关闭完成");
    Ok(())
}

pub async fn init(state: Arc<AppState>) -> anyhow::Result<()> {
    tracing::info!("开始初始化定时任务");

    let scheduler = JobScheduler::new().await?;

    task::build(state)?.register_to(&scheduler).await?;

    SCHEDULER
        .set(scheduler)
        .map_err(|_| anyhow::anyhow!("重复初始化定时任务"))
}
