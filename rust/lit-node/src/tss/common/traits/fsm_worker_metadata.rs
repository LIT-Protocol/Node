use std::fmt::Debug;

/// This trait describes cross-cutting functionality that is needed by the FSM worker.
///
/// The concept of a "lifecycle" refers to each loop that the FSM worker goes through. It is
/// recommended that the lifecycle ID be updated during (or at the beginning of) each loop.
#[async_trait::async_trait]
pub trait FSMWorkerMetadata: Debug + Send + Sync {
    type LifecycleId;

    /// Returns the current lifecycle ID.
    fn get_lifecycle_id(&self) -> Self::LifecycleId;

    /// Updates the lifecycle ID. This may be used to derive an ID
    /// for the next epoch change. If `new_id` is `Some`, then the lifecycle ID will be updated
    /// to the value of `new_id`. Otherwise, the lifecycle ID will be updated to the next value,
    /// which depends on the underlying implementation.
    fn update_lifecycle_id(&self, new_id: Option<Self::LifecycleId>);

    /// This function will compare the peer's metadata with this node's. Calling this function
    /// may result in all instances of the `yield_until_update` future to be able to be polled.
    fn compare_with_peer(&self, peer_lifecycle_id: Self::LifecycleId);

    /// This function yields to the async runtime until an update is needed, in which case it will
    /// return an `Ok` with the new metadata this node should update to. Otherwise, it will return
    /// an `Err`
    async fn yield_until_update(&self) -> Self::LifecycleId;
}
