use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;
use once_cell::sync::Lazy;

use lit_blockchain::contracts::release_register::Release;

pub static CACHE: Lazy<Cache<String, Arc<dyn Any + Send + Sync>>> = Lazy::new(|| {
    Cache::builder()
    .max_capacity(1024)
    .time_to_live(Duration::from_secs(270)) // 4.5 Minutes.
    .build()
});

pub trait CacheValue {
    fn as_string(&self) -> Option<&String>;
    fn as_bool(&self) -> Option<&bool>;
    fn as_release(&self) -> Option<&Release>;
}

impl CacheValue for Arc<dyn Any + Send + Sync> {
    fn as_string(&self) -> Option<&String> {
        self.downcast_ref()
    }

    fn as_bool(&self) -> Option<&bool> {
        self.downcast_ref()
    }

    fn as_release(&self) -> Option<&Release> {
        self.downcast_ref()
    }
}
