pub mod storage;

use std::borrow::Cow;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::storage::cache::storage::{CacheSetMode, CacheStorage};
use crate::util::time::UnixTimestampSecs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cache<T> {
    /// 缓存ID
    pub id: String,
    /// 创建时间
    pub created_at: i64,
    /// 过期时间
    pub expires_at: i64,
    /// 缓存类型
    pub kind: String,
    /// 缓存数据
    pub data: T,
}

pub trait CacheData: DeserializeOwned + Serialize + Send + Sync {
    fn kind() -> &'static str;

    fn generate_id(&self) -> Cow<'_, str> {
        crate::util::uuid::v4().into()
    }

    fn with_ttl(self, ttl: Duration) -> Cache<Self> {
        Cache::with_ttl(self, ttl)
    }
}

impl<T> Cache<T>
where
    T: CacheData,
{
    pub fn new(data: T, created_at: i64, expires_at: i64) -> Self {
        Self {
            id: data.generate_id().into_owned(),
            created_at,
            expires_at,
            kind: T::kind().to_owned(),
            data,
        }
    }

    pub fn with_ttl(data: T, ttl: Duration) -> Self {
        let now = UnixTimestampSecs::now();
        Self::new(data, now.as_i64(), now.add(ttl).as_i64())
    }

    pub async fn get_in<S>(id: &str, storage: &S) -> anyhow::Result<Option<Self>>
    where
        S: CacheStorage,
    {
        storage.get(id).await
    }

    pub async fn get_ttl_in<S>(id: &str, storage: &S) -> anyhow::Result<Option<Duration>>
    where
        S: CacheStorage,
    {
        let Some(expires_at) = Self::get_expires_at_in(id, storage).await? else {
            return Ok(None);
        };
        Ok(expires_at
            .checked_sub(UnixTimestampSecs::now().as_i64())
            .filter(|time| *time > 0)
            .map(|time| Duration::from_secs(time as u64)))
    }

    pub async fn get_expires_at_in<S>(id: &str, storage: &S) -> anyhow::Result<Option<i64>>
    where
        S: CacheStorage,
    {
        storage.get_expires_at::<T>(id).await
    }

    pub async fn set_in<S>(&self, mode: CacheSetMode, storage: &S) -> anyhow::Result<bool>
    where
        S: CacheStorage,
    {
        storage.set(self, mode).await
    }

    pub async fn set_ttl_in<S>(id: &str, ttl: Duration, storage: &S) -> anyhow::Result<bool>
    where
        S: CacheStorage,
    {
        Self::set_expires_at_in(id, UnixTimestampSecs::now().add(ttl).as_i64(), storage).await
    }

    pub async fn set_expires_at_in<S>(
        id: &str,
        expires_at: i64,
        storage: &S,
    ) -> anyhow::Result<bool>
    where
        S: CacheStorage,
    {
        storage.set_expires_at::<T>(id, expires_at).await
    }

    pub async fn exists_in<S>(id: &str, storage: &S) -> anyhow::Result<bool>
    where
        S: CacheStorage,
    {
        storage.exists::<T>(id).await
    }

    pub async fn remove_in<S>(id: &str, storage: &S) -> anyhow::Result<()>
    where
        S: CacheStorage,
    {
        storage.remove::<T>(id).await
    }

    pub async fn batch_remove_in<S>(id_prefix: &str, storage: &S) -> anyhow::Result<()>
    where
        S: CacheStorage,
    {
        storage.batch_remove::<T>(id_prefix).await
    }

    pub async fn get(id: &str) -> anyhow::Result<Option<Self>> {
        Self::get_in(id, storage::get()).await
    }

    pub async fn get_ttl(id: &str) -> anyhow::Result<Option<Duration>> {
        Self::get_ttl_in(id, storage::get()).await
    }

    pub async fn get_expires_at(id: &str) -> anyhow::Result<Option<i64>> {
        Self::get_expires_at_in(id, storage::get()).await
    }

    pub async fn set(&self, mode: CacheSetMode) -> anyhow::Result<bool> {
        Self::set_in(self, mode, storage::get()).await
    }

    pub async fn set_ttl(id: &str, ttl: Duration) -> anyhow::Result<bool> {
        Self::set_ttl_in(id, ttl, storage::get()).await
    }

    pub async fn set_expires_at(id: &str, expires_at: i64) -> anyhow::Result<bool> {
        Self::set_expires_at_in(id, expires_at, storage::get()).await
    }

    pub async fn exists(id: &str) -> anyhow::Result<bool> {
        Self::exists_in(id, storage::get()).await
    }

    pub async fn remove(id: &str) -> anyhow::Result<()> {
        Self::remove_in(id, storage::get()).await
    }

    pub async fn batch_remove(id_prefix: &str) -> anyhow::Result<()> {
        Self::batch_remove_in(id_prefix, storage::get()).await
    }

    pub fn is_expired(&self) -> bool {
        UnixTimestampSecs::now().as_i64() > self.expires_at
    }
}
