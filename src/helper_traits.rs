use std::sync::atomic::Ordering;

/// This trait determines the type of marker to be used for the buffer queue
pub trait MarkerData {
    /// The type of marker -- implements default(this is used to set the value during
    /// initialization)
    type MarkerType: MarkerAtomicOperations + Default;
}

/// Used to define functions for markers to perform operations on them
pub(crate) trait MarkerAtomicOperations {
    type OutputItem;
    fn load(&self, order: Ordering) -> Self::OutputItem;
    fn store(&self, val: Self::OutputItem, order: Ordering);
    fn fetch_add(&self) {}
}
