use std::sync::atomic::Ordering;

/// This trait determines the type of marker to be used for the buffer queue
pub trait MarkerData {
    /// The type of marker -- implements default(this is used to set the value during
    /// initialization)
    type MarkerType: MarkerAtomicOperations + Default;

    // Used in calculations
    const DENOMINATOR: usize;
}

/// Used to define functions for markers to perform operations on them
pub(crate) trait MarkerAtomicOperations {
    type OutputItem: OutputTrait + Copy;
    fn load(&self, order: Ordering) -> Self::OutputItem;
    fn store(&self, val: Self::OutputItem, order: Ordering);
    fn fetch_add(&self, val: usize, order: Ordering) -> usize;

    fn fetch_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<Self::OutputItem, Self::OutputItem>;
}

// To convert to/from usize
pub trait OutputTrait {
    fn to_usize(self) -> usize;
    fn from_usize(val: usize) -> Self;
    fn wrapping_increment(self, boundary: usize) -> Self;
}
