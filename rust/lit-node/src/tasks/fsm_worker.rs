use arc_swap::ArcSwap;
use std::sync::{atomic::AtomicU64, Arc};

use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;

#[derive(Debug)]
pub struct CounterBasedFSMWorkerMetadata {
    lifecycle_id: ArcSwap<u64>,
    greatest_peer_lifecycle_id: AtomicU64,
}

impl Default for CounterBasedFSMWorkerMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterBasedFSMWorkerMetadata {
    pub fn new() -> Self {
        Self {
            lifecycle_id: ArcSwap::new(Arc::new(0)),
            greatest_peer_lifecycle_id: 0.into(),
        }
    }
}

#[async_trait::async_trait]
impl FSMWorkerMetadata for CounterBasedFSMWorkerMetadata {
    type LifecycleId = u64;

    fn get_lifecycle_id(&self) -> Self::LifecycleId {
        *self.lifecycle_id.load_full()
    }

    fn update_lifecycle_id(&self, new_id: Option<Self::LifecycleId>) {
        match new_id {
            Some(id) => self.lifecycle_id.store(Arc::new(id)),
            None => {
                // Increment the lifecycle id
                self.lifecycle_id
                    .store(Arc::new(self.get_lifecycle_id() + 1));
            }
        }
    }

    fn compare_with_peer(&self, peer_lifecycle_id: Self::LifecycleId) {
        if peer_lifecycle_id
            > self
                .greatest_peer_lifecycle_id
                .load(std::sync::atomic::Ordering::SeqCst)
        {
            self.greatest_peer_lifecycle_id
                .store(peer_lifecycle_id, std::sync::atomic::Ordering::SeqCst);
        }
    }

    async fn yield_until_update(&self) -> Self::LifecycleId {
        // Every 500ms, check if the greatest peer lifecycle ID is greater than the current lifecycle ID.
        loop {
            let my_lifecycle_id = self.get_lifecycle_id();
            let greatest_peer_lifecycle_id = self
                .greatest_peer_lifecycle_id
                .load(std::sync::atomic::Ordering::SeqCst);

            if greatest_peer_lifecycle_id > my_lifecycle_id {
                info!(
                    "Peer lifecycle ID {} is greater than my lifecycle ID {}, updating metadata",
                    greatest_peer_lifecycle_id, my_lifecycle_id
                );
                return greatest_peer_lifecycle_id;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}

#[derive(Debug)]
pub struct NoopFSMWorkerMetadata;

impl Default for NoopFSMWorkerMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopFSMWorkerMetadata {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl FSMWorkerMetadata for NoopFSMWorkerMetadata {
    type LifecycleId = u64;

    fn get_lifecycle_id(&self) -> Self::LifecycleId {
        1
    }

    fn update_lifecycle_id(&self, _new_id: Option<Self::LifecycleId>) {}

    fn compare_with_peer(&self, _peer_lifecycle_id: Self::LifecycleId) {}

    async fn yield_until_update(&self) -> Self::LifecycleId {
        // Sleep for 100s so it never returns
        tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
        0
    }
}
