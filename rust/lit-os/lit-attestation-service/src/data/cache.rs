use lit_attestation::Attestation;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::any::Any;
use std::sync::Arc;

pub static CACHE: Lazy<Cache<String, Arc<dyn Any + Send + Sync>>> = Lazy::new(|| Cache::new(10000));

pub trait CacheValue {
    fn as_string(&self) -> Option<&String>;
    fn as_attestation(&self) -> Option<&Attestation>;
}

impl CacheValue for Arc<dyn Any + Send + Sync> {
    fn as_string(&self) -> Option<&String> {
        self.downcast_ref()
    }

    fn as_attestation(&self) -> Option<&Attestation> {
        self.downcast_ref()
    }
}

pub(crate) fn cache_key_session<S>(session_id: S) -> String
where
    S: AsRef<str>,
{
    format!("session::{}", session_id.as_ref())
}

#[cfg(test)]
mod tests {
    use crate::data::cache::CacheValue;
    use std::sync::Arc;

    use super::CACHE;

    #[tokio::test]
    async fn test_cache_exists_set_exists() {
        assert!(CACHE.get("test_cache_exists_set_exists").await.is_none());
        CACHE.insert("test_cache_exists_set_exists".into(), Arc::new("456".to_string())).await;
        assert!(CACHE.get("test_cache_exists_set_exists").await.is_some());
    }

    #[tokio::test]
    async fn test_cache_get_set_get() {
        assert!(CACHE.get("test_cache_get_set_get").await.is_none());
        CACHE.insert("test_cache_get_set_get".into(), Arc::new("456".to_string())).await;
        match CACHE.get("test_cache_get_set_get").await {
            None => panic!("CACHE.get(): expected Some"),
            Some(v) => {
                assert_eq!(v.as_string(), Some(&"456".to_string()));
            }
        }
    }

    #[tokio::test]
    async fn test_cache_set_del_get() {
        CACHE.insert("test_cache_set_del_get".into(), Arc::new("456".to_string())).await;
        CACHE.invalidate(&"test_cache_set_del_get".to_string()).await;
        assert!(CACHE.get("test_cache_set_del_get").await.is_none());
    }
}
