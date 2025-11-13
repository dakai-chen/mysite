use serde::{Deserialize, Serialize};

use crate::model::bo::system::{InfoBo, InfoCpuBo, InfoDatabaseBo, InfoDiskBo, InfoSystemBo};

#[derive(Debug, Deserialize)]
pub struct SetLogLevelDto {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct SetShutdownTimeoutDto {
    pub timeout: u64,
}

#[derive(Debug, Serialize)]
pub struct InfoDto {
    /// 系统信息
    pub system: InfoSystemDto,
    /// 硬盘信息
    pub disks: Vec<InfoDiskDto>,
    /// 数据库信息
    pub database: InfoDatabaseDto,
}

impl From<InfoBo> for InfoDto {
    fn from(value: InfoBo) -> Self {
        Self {
            system: value.system.into(),
            disks: value.disks.into_iter().map(InfoDiskDto::from).collect(),
            database: value.database.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InfoDatabaseDto {
    /// 数据库连接状态
    pub state: String,
    /// 空闲连接数
    pub idle_connections: usize,
    /// 总连接数
    pub total_connections: u32,
}

impl From<InfoDatabaseBo> for InfoDatabaseDto {
    fn from(value: InfoDatabaseBo) -> Self {
        Self {
            state: value.state,
            idle_connections: value.idle_connections,
            total_connections: value.total_connections,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InfoSystemDto {
    /// CPU 各核心信息
    pub cpus: Vec<InfoCpuDto>,
    /// CPU 整体使用率
    pub global_cpu_usage: f32,
    /// CPU 架构
    pub cpu_arch: String,
    /// 内存大小
    pub total_memory: u64,
    /// 空闲的内存大小
    pub free_memory: u64,
    /// 可用的内存大小
    pub available_memory: u64,
    /// 已用的内存大小
    pub used_memory: u64,
    /// 交换分区大小
    pub total_swap: u64,
    /// 空闲的交换分区大小
    pub free_swap: u64,
    /// 已用的交换分区大小
    pub used_swap: u64,
    /// 操作系统版本
    pub os_version: Option<String>,
    /// 内核版本
    pub kernel_version: String,
}

impl From<InfoSystemBo> for InfoSystemDto {
    fn from(value: InfoSystemBo) -> Self {
        Self {
            cpus: value.cpus.into_iter().map(InfoCpuDto::from).collect(),
            global_cpu_usage: value.global_cpu_usage,
            cpu_arch: value.cpu_arch,
            total_memory: value.total_memory,
            free_memory: value.free_memory,
            available_memory: value.available_memory,
            used_memory: value.used_memory,
            total_swap: value.total_swap,
            free_swap: value.free_swap,
            used_swap: value.used_swap,
            os_version: value.os_version,
            kernel_version: value.kernel_version,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InfoCpuDto {
    /// CPU 名称
    pub name: String,
    /// CPU 品牌
    pub brand: String,
    /// CPU 使用率
    pub usage: f32,
}

impl From<InfoCpuBo> for InfoCpuDto {
    fn from(value: InfoCpuBo) -> Self {
        Self {
            name: value.name,
            brand: value.brand,
            usage: value.usage,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InfoDiskDto {
    /// 硬盘名称
    pub name: String,
    /// 硬盘使用的文件系统
    pub file_system: String,
    /// 硬盘的挂载点
    pub mount_point: String,
    /// 硬盘大小
    pub total_space: u64,
    /// 可用的硬盘大小
    pub available_space: u64,
}

impl From<InfoDiskBo> for InfoDiskDto {
    fn from(value: InfoDiskBo) -> Self {
        Self {
            name: value.name,
            file_system: value.file_system,
            mount_point: value.mount_point,
            total_space: value.total_space,
            available_space: value.available_space,
        }
    }
}
