mod db;
pub use db::{get, init};

use crate::storage::cache::{Cache, CacheData};

pub trait CacheStorage {
    fn get<T>(&self, id: &str) -> impl Future<Output = anyhow::Result<Option<Cache<T>>>> + Send
    where
        T: CacheData;

    fn get_expires_at<T>(
        &self,
        id: &str,
    ) -> impl Future<Output = anyhow::Result<Option<i64>>> + Send
    where
        T: CacheData;

    fn set<T>(
        &self,
        cache: &Cache<T>,
        mode: CacheSetMode,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send
    where
        T: CacheData;

    fn set_expires_at<T>(
        &self,
        id: &str,
        expires_at: i64,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send
    where
        T: CacheData;

    fn exists<T>(&self, id: &str) -> impl Future<Output = anyhow::Result<bool>> + Send
    where
        T: CacheData;

    fn remove<T>(&self, id: &str) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        T: CacheData;

    fn batch_remove<T>(&self, id_prefix: &str) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        T: CacheData;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheSetMode {
    /// 无条件覆盖写入
    /// 无论缓存中是否已存在该缓存类型的 ID ，直接写入/更新缓存，并设置过期时间
    Overwrite,
    /// 仅当缓存中不存在该缓存类型的 ID 时才写入（不存在则新增，存在则忽略）
    /// 用于避免并发场景下的重复写入
    OnlyIfNotExists,
    /// 仅当缓存中已存在该缓存类型的 ID 时才更新（存在则覆盖，不存在则忽略）
    /// 用于仅更新已有的缓存数据，避免新增无效缓存
    OnlyIfExists,
}
