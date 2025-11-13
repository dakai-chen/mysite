#[derive(Debug)]
pub struct InfoBo {
    /// 系统信息
    pub system: InfoSystemBo,
    /// 硬盘信息
    pub disks: Vec<InfoDiskBo>,
    /// 数据库信息
    pub database: InfoDatabaseBo,
}

#[derive(Debug)]
pub struct InfoDatabaseBo {
    /// 数据库连接状态
    pub state: String,
    /// 空闲连接数
    pub idle_connections: usize,
    /// 总连接数
    pub total_connections: u32,
}

#[derive(Debug)]
pub struct InfoSystemBo {
    /// CPU 各核心信息
    pub cpus: Vec<InfoCpuBo>,
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

impl From<&sysinfo::System> for InfoSystemBo {
    fn from(system: &sysinfo::System) -> Self {
        Self {
            cpus: system.cpus().iter().map(InfoCpuBo::from).collect(),
            global_cpu_usage: system.global_cpu_usage(),
            cpu_arch: sysinfo::System::cpu_arch(),
            total_memory: system.total_memory(),
            free_memory: system.free_memory(),
            available_memory: system.available_memory(),
            used_memory: system.used_memory(),
            total_swap: system.total_swap(),
            free_swap: system.free_swap(),
            used_swap: system.used_swap(),
            os_version: sysinfo::System::long_os_version(),
            kernel_version: sysinfo::System::kernel_long_version(),
        }
    }
}

#[derive(Debug)]
pub struct InfoCpuBo {
    /// CPU 名称
    pub name: String,
    /// CPU 品牌
    pub brand: String,
    /// CPU 使用率
    pub usage: f32,
}

impl From<&sysinfo::Cpu> for InfoCpuBo {
    fn from(cpu: &sysinfo::Cpu) -> Self {
        Self {
            name: cpu.name().to_owned(),
            brand: cpu.brand().trim().to_owned(),
            usage: cpu.cpu_usage(),
        }
    }
}

#[derive(Debug)]
pub struct InfoDiskBo {
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

impl From<&sysinfo::Disk> for InfoDiskBo {
    fn from(disk: &sysinfo::Disk) -> Self {
        Self {
            name: disk.name().to_string_lossy().into_owned(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        }
    }
}
